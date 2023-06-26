use http::Method;
use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

pub fn get_layers() -> (
    CorsLayer,
    // Don't let this scare you!  This is what makes our Trace understand that Axum errors are Failures
    TraceLayer<SharedClassifier<ServerErrorsAsFailures>>,
) {
    let cors_layer = CorsLayer::new()
        //.allow_origin(addr.to_string().parse::<HeaderValue>().unwrap()) // Proper CORS handling
        .allow_origin(Any) // for convenience
        .allow_headers([http::header::CONTENT_TYPE])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ]);

    let trace_layer = TraceLayer::new_for_http();
    (cors_layer, trace_layer)
}
