use serde_json::Value;
use socketioxide::extract::{Bin, Data, SocketRef};
use tracing::info;

pub fn on_connection(socket: SocketRef, Data(data): Data<Value>) {
    info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);
    socket.emit("auth", data).ok();

    socket.on("message", on_event);
}

fn on_event(socket: SocketRef, Data(data): Data<Value>, Bin(bin): Bin) {
    info!("Received event: {:?} {:?}", data, bin);
    socket.bin(bin).emit("message-back", data).ok();
}
