#[macro_use]
extern crate lazy_static;

mod beacon;
mod config;
mod helpers;
mod metrics;
mod node;
mod url_query;

use beacon_chain::{BeaconChain, BeaconChainTypes};
pub use config::Config as ApiConfig;
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Body, Method, Response, Server, StatusCode};
use slog::{info, o, warn};
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::runtime::TaskExecutor;
use url_query::UrlQuery;

#[derive(PartialEq, Debug)]
pub enum ApiError {
    MethodNotAllowed(String),
    ServerError(String),
    NotImplemented(String),
    InvalidQueryParams(String),
    NotFound(String),
    ImATeapot(String), // Just in case.
}

pub type ApiResult = Result<Response<Body>, ApiError>;

impl Into<Response<Body>> for ApiError {
    fn into(self) -> Response<Body> {
        let status_code: (StatusCode, String) = match self {
            ApiError::MethodNotAllowed(desc) => (StatusCode::METHOD_NOT_ALLOWED, desc),
            ApiError::ServerError(desc) => (StatusCode::INTERNAL_SERVER_ERROR, desc),
            ApiError::NotImplemented(desc) => (StatusCode::NOT_IMPLEMENTED, desc),
            ApiError::InvalidQueryParams(desc) => (StatusCode::BAD_REQUEST, desc),
            ApiError::NotFound(desc) => (StatusCode::NOT_FOUND, desc),
            ApiError::ImATeapot(desc) => (StatusCode::IM_A_TEAPOT, desc),
        };
        Response::builder()
            .status(status_code.0)
            .body(Body::from(status_code.1))
            .expect("Response should always be created.")
    }
}

impl From<store::Error> for ApiError {
    fn from(e: store::Error) -> ApiError {
        ApiError::ServerError(format!("Database error: {:?}", e))
    }
}

impl From<types::BeaconStateError> for ApiError {
    fn from(e: types::BeaconStateError) -> ApiError {
        ApiError::ServerError(format!("BeaconState error: {:?}", e))
    }
}

impl From<state_processing::per_slot_processing::Error> for ApiError {
    fn from(e: state_processing::per_slot_processing::Error) -> ApiError {
        ApiError::ServerError(format!("PerSlotProcessing error: {:?}", e))
    }
}

pub fn start_server<T: BeaconChainTypes + Clone + 'static>(
    config: &ApiConfig,
    executor: &TaskExecutor,
    beacon_chain: Arc<BeaconChain<T>>,
    db_path: PathBuf,
    log: &slog::Logger,
) -> Result<exit_future::Signal, hyper::Error> {
    let log = log.new(o!("Service" => "Api"));

    // build a channel to kill the HTTP server
    let (exit_signal, exit) = exit_future::signal();

    let exit_log = log.clone();
    let server_exit = exit.and_then(move |_| {
        info!(exit_log, "API service shutdown");
        Ok(())
    });

    let db_path = DBPath(db_path);

    // Get the address to bind to
    let bind_addr = (config.listen_address, config.port).into();

    // Clone our stateful objects, for use in service closure.
    let server_log = log.clone();
    let server_bc = beacon_chain.clone();

    let service = move || {
        let log = server_log.clone();
        let beacon_chain = server_bc.clone();
        let db_path = db_path.clone();

        // Create a simple handler for the router, inject our stateful objects into the request.
        service_fn_ok(move |mut req| {
            metrics::inc_counter(&metrics::REQUEST_COUNT);
            let timer = metrics::start_timer(&metrics::REQUEST_RESPONSE_TIME);

            req.extensions_mut().insert::<slog::Logger>(log.clone());
            req.extensions_mut()
                .insert::<Arc<BeaconChain<T>>>(beacon_chain.clone());
            req.extensions_mut().insert::<DBPath>(db_path.clone());

            let path = req.uri().path().to_string();

            // Route the request to the correct handler.
            let result = match (req.method(), path.as_ref()) {
                (&Method::GET, "/beacon/state") => beacon::get_state::<T>(req),
                (&Method::GET, "/beacon/state_root") => beacon::get_state_root::<T>(req),
                (&Method::GET, "/metrics") => metrics::get_prometheus::<T>(req),
                (&Method::GET, "/node/version") => node::get_version(req),
                (&Method::GET, "/node/genesis_time") => node::get_genesis_time::<T>(req),
                _ => Err(ApiError::MethodNotAllowed(path.clone())),
            };

            let response = match result {
                // Return the `hyper::Response`.
                Ok(response) => {
                    metrics::inc_counter(&metrics::SUCCESS_COUNT);
                    slog::debug!(log, "Request successful: {:?}", path);
                    response
                }
                // Map the `ApiError` into `hyper::Response`.
                Err(e) => {
                    slog::debug!(log, "Request failure: {:?}", path);
                    e.into()
                }
            };

            metrics::stop_timer(timer);

            response
        })
    };

    let log_clone = log.clone();
    let server = Server::bind(&bind_addr)
        .serve(service)
        .with_graceful_shutdown(server_exit)
        .map_err(move |e| {
            warn!(
                log_clone,
                "API failed to start, Unable to bind"; "address" => format!("{:?}", e)
            )
        });

    info!(
        log,
        "REST API started";
        "address" => format!("{}", config.listen_address),
        "port" => config.port,
    );

    executor.spawn(server);

    Ok(exit_signal)
}

fn success_response(body: Body) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .body(body)
        .expect("We should always be able to make response from the success body.")
}

#[derive(Clone)]
pub struct DBPath(PathBuf);

impl Deref for DBPath {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
