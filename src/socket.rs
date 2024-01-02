use serde_json::Value;
use socketioxide::extract::SocketRef;
use tracing::info;

pub fn on_event(socket: &SocketRef, data: Value, bin: Vec<Vec<u8>>) {
    info!("Received event: {:?} {:?}", data, bin);
    socket.bin(bin).emit("message-back", data).ok();
}
