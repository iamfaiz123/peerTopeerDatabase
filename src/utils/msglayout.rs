
use std::net::SocketAddr;
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Message {
    Handshake { node_name: String, tcp_addr: SocketAddr },
    Greeting,
    Heartbeat,
    HeartbeatResponse,
    SetValue { key: String, value: String },
    GetValue { key: String },
    ValueResponse { value: Option<String> },
    Sync { key: String, value: String },
}