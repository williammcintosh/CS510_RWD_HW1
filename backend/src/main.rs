use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
};

use dotenvy::dotenv;
use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;
mod http_error;
//mod layers;
mod db;
mod layers;
mod routes;

#[tokio::main]
async fn main() {
    // Collect environment variables from a .env file
    dotenv().ok();
    // Initialize tokio's logging/tracing
    init_logging();
    debug!("Application initialized.");

    // let resp = reqwest::get("https://httpbin.org/ip")
    //     .await?
    //     .json::<HashMap<String, String>>()
    //     .await?;
    // info!("{:#?}", resp);

    // This runs our listen server
    run().await;
}

async fn run() {
    let addr = get_host_from_env();

    info!("Listening on {}", addr);

    let (cors_layer, trace_layer) = layers::get_layers();

    let app = routes::get_router().layer(cors_layer).layer(trace_layer);

    // Start the server!
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Rust server died, must be hardware issue");
}

fn init_logging() {
    // https://github.com/tokio-rs/axum/blob/main/examples/tracing-aka-logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "backend=trace,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

// Basic utility fn for creating a Rusty-address from .env vars
fn get_host_from_env() -> SocketAddr {
    let host = std::env::var("API_HOST").expect("Could not find API_HOST from environment");
    let api_host =
        IpAddr::from_str(&host).expect("Could not create viable IP Address from API_HOST");
    let api_port: u16 = std::env::var("API_PORT")
        .expect("Could not find API_PORT from environment")
        .parse()
        .expect("Could not create viable port from API_PORT environment variable.");

    SocketAddr::from((api_host, api_port))
}
