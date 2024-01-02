pub mod socket;

use axum::{http::StatusCode, routing::get, Router};
use serde_json::Value;
use socketioxide::{
    extract::{Bin, Data, SocketRef},
    SocketIo,
};
use tracing::info;

fn on_connect(socket: SocketRef, Data(data): Data<Value>) {
    info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);
    socket.emit("auth", data).ok();

    socket.on(
        "message",
        |socket: SocketRef, Data::<Value>(data), Bin(bin)| {
            socket::on_event(&socket, data, bin);
        },
    );
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub fn app() -> Router {
    let (layer, io) = SocketIo::new_layer();

    io.ns("/", on_connect);
    io.ns("/custom", on_connect);

    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health", get(health_check))
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
