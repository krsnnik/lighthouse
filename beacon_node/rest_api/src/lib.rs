extern crate futures;
extern crate hyper;
#[macro_use]
mod macros;
mod api_request;
mod beacon_chain_api;
mod beacon_node;
pub mod config;

use beacon_chain::{BeaconChain, BeaconChainTypes};
pub use config::Config as ApiConfig;

use slog::{info, o, warn};
use std::sync::Arc;
use tokio::runtime::TaskExecutor;

use crate::beacon_node::BeaconNodeServiceInstance;
use api_request::ApiRequest;
use hyper::rt::Future;
use hyper::service::{service_fn, Service};
use hyper::{Body, Request, Response, Server, StatusCode};
use hyper_router::{RouterBuilder, RouterService};

#[derive(PartialEq, Debug)]
pub enum ApiError {
    MethodNotAllowed { desc: String },
    ServerError { desc: String },
    NotImplemented { desc: String },
    InvalidQueryParams { desc: String },
    ImATeapot { desc: String }, // Just in case.
}

pub type ApiResult = Result<Response<Body>, ApiError>;

impl Into<Response<Body>> for ApiError {
    fn into(self) -> Response<Body> {
        let status_code: (StatusCode, String) = match self {
            ApiError::MethodNotAllowed { desc } => (StatusCode::METHOD_NOT_ALLOWED, desc),
            ApiError::ServerError { desc } => (StatusCode::INTERNAL_SERVER_ERROR, desc),
            ApiError::NotImplemented { desc } => (StatusCode::NOT_IMPLEMENTED, desc),
            ApiError::InvalidQueryParams { desc } => (StatusCode::BAD_REQUEST, desc),
            ApiError::ImATeapot { desc } => (StatusCode::IM_A_TEAPOT, desc),
        };
        Response::builder()
            .status(status_code.0)
            .body(Body::from(status_code.1))
            .expect("Response should always be created.")
    }
}

pub trait ApiService {
    fn add_routes(&mut self, router_builder: RouterBuilder) -> Result<RouterBuilder, hyper::Error>;
}

pub fn start_server<T: BeaconChainTypes + Clone + 'static>(
    config: &ApiConfig,
    executor: &TaskExecutor,
    beacon_chain: Arc<BeaconChain<T>>,
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

    // Get the address to bind to
    let bind_addr = (config.listen_address, config.port).into();

    // Clone our stateful objects, for use in service closure.
    let server_log = log.clone();
    let server_bc = beacon_chain.clone();

    // Create the service closure
    let service = move || {
        //TODO: This router must be moved out of this closure, so it isn't rebuilt for every connection.
        let mut router = build_router_service::<T>();

        // Clone our stateful objects, for use in handler closure
        let service_log = server_log.clone();
        let service_bc = server_bc.clone();

        // Create a simple handler for the router, inject our stateful objects into the request.
        service_fn(move |mut req| {
            req.extensions_mut()
                .insert::<slog::Logger>(service_log.clone());
            req.extensions_mut()
                .insert::<Arc<BeaconChain<T>>>(service_bc.clone());
            router.call(req)
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

fn build_router_service<T: BeaconChainTypes + 'static>() -> RouterService {
    let mut router_builder = RouterBuilder::new();

    let mut bn_service: BeaconNodeServiceInstance<T> = BeaconNodeServiceInstance {
        marker: std::marker::PhantomData,
    };

    router_builder = bn_service
        .add_routes(router_builder)
        .expect("The routes should always be made.");

    RouterService::new(router_builder.build())
}

fn path_from_request(req: &Request<Body>) -> String {
    req.uri()
        .path_and_query()
        .as_ref()
        .map(|pq| String::from(pq.as_str()))
        .unwrap_or(String::new())
}

fn success_response(body: Body) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .body(body)
        .expect("We should always be able to make response from the success body.")
}
