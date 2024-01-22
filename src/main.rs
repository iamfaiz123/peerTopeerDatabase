use tokio::net::UdpSocket;
use std::net::SocketAddr;
use p2p_database::utils::node::broadcast_me;
use tokio::sync::RwLock;
use std::collections::HashMap;
use p2p_database::utils::node::NodeInfo;

#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>> {
    let local_addr: SocketAddr = "0.0.0.0:8888".parse()?;
    let socket = UdpSocket::bind(&local_addr).await?;
    socket.set_broadcast(true)?;
    let socket = std::sync::Arc::new(socket);

    // create a new variable to store information about other nodes
    let nodes = RwLock::new(HashMap::<String,NodeInfo>::new());
    // broadcast a handshake message to other nodes
    broadcast_me(socket.clone());
    loop{}
    Ok(())
}
