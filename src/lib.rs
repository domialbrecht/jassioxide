pub mod handlers;

use axum::{routing::get, Router};
use socketioxide::SocketIo;
use tracing::info;

pub fn app() -> Router {
    let (layer, io) = SocketIo::new_layer();

    io.ns("/", handlers::socket::on_connection);

    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health", get(handlers::api::health_check))
        .layer(layer)
}

#[tokio::main]
pub async fn run() -> anyhow::Result<()> {
    info!("Starting server");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Bind to port");
    axum::serve(listener, app()).await.expect("Run server");

    Ok(())
}
