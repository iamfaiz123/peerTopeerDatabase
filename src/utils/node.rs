use std::net::SocketAddr;
use mac_address::MacAddressError;
use tokio::net::UdpSocket;
use crate::utils::msglayout::Message;
use tokio::sync::RwLock;
use std::collections::HashMap;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
pub struct NodeInfo {
    last_seen: std::time::Instant,
    // node communicate to other nodes using tcp
    tcp_addr: SocketAddr,
}



// function to get mac address of each node, since all node will be present in same network
fn get_mac_address() -> Result<String, MacAddressError> {

    let mac = mac_address::get_mac_address()?;

    match mac {
        Some(address) => Ok(address.to_string()),
        None => Err(MacAddressError::InternalError),
    }
}


// broadcast me tell the presence of current node in the other machines
pub fn broadcast_me(machine_socket:std::sync::Arc<UdpSocket>){
    
    // start a task to broadcast machine , this will run in loop and will be closed when the program is killed
    tokio::spawn(async move {
        match get_mac_address() {
            Ok(node_name) => {
                let tcp_addr = format!("{}:{}", "0.0.0.0", "8080").parse().unwrap();
                let msg = Message::Handshake {
                    node_name: node_name.clone(),
                    tcp_addr,
                };
        let serialized_msg = serde_json::to_string(&msg).unwrap();

        loop {
            println!("Sending UDP broadcast...");
            machine_socket.send_to(serialized_msg.as_bytes(), "255.255.255.255").await.unwrap();
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
            },
            Err(e) => {
                eprintln!("Error fetching MAC address: {:?}", e);
            }
        }
    });
}

pub async fn listen_nodes(socket:&UdpSocket,nodes:RwLock<HashMap<String,NodeInfo>>)->Result<(),std::io::Error>{
    let mut buf = vec![0u8; 1024];
    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        println!("Received data on UDP from {}", addr);
        let received_msg: Message = serde_json::from_slice(&buf[..len])?;

        let local_node_name = get_mac_address().unwrap();

        if let Message::Handshake { node_name, tcp_addr } = received_msg {
            // Ignore packets from ourselves
            if node_name == local_node_name {
                continue;
            }
            println!("Received handshake from: {}", node_name);
            {
                let mut nodes_guard = nodes.write().await;
                nodes_guard.insert(node_name.clone(), NodeInfo { last_seen: std::time::Instant::now(), tcp_addr });
            }

            let greeting = Message::Greeting;
            let serialized_greeting = serde_json::to_string(&greeting).unwrap();
            socket.send_to(serialized_greeting.as_bytes(), &addr).await?;

            // Start heartbeat for this node
            tokio::spawn(async move {
                loop {
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    println!("Sending heartbeat to {}", tcp_addr);
                    let mut stream = TcpStream::connect(tcp_addr).await.unwrap();
                    let heartbeat_msg = Message::Heartbeat;
                    let serialized_msg = serde_json::to_string(&heartbeat_msg).unwrap();
                    stream.write_all(serialized_msg.as_bytes()).await.unwrap();
                }
            });
        }
    }
}