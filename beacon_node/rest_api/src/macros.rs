macro_rules! result_to_response {
    ($handler: path) => {
        |req: hyper::Request<Body>| -> hyper::Response<Body> {
            let log = req
                .extensions()
                .get::<slog::Logger>()
                .expect("Our logger should be on req.")
                .clone();

            let path = crate::path_from_request(&req);
            let result = $handler(req);

            match result {
                Ok(response) => {
                    slog::debug!(log, "Request successful: {:?}", path);
                    response
                }
                Err(e) => {
                    slog::debug!(log, "Request failure: {:?}", path);
                    e.into()
                }
            }
        }
    };
}
