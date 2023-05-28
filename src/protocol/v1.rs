use std::net::IpAddr;
use anyhow::{Result};
use tokio::{net::{TcpStream}};
use tokio::io::{AsyncWriteExt};


pub async fn proxy_protocolv1(client_stream: &TcpStream, serveur_stream: &mut TcpStream) -> Result<()>{
    let streamaddr = client_stream.local_addr().expect("Failed to get local_addr");

    match client_stream.peer_addr() {
        Ok(peeraddr) => match peeraddr.ip() {
            IpAddr::V4(ip4) => {
                serveur_stream.write_all(format!("PROXY TCP4 {} {} {} {}\r\n", ip4.to_string(), streamaddr.ip().to_string(), peeraddr.port(), streamaddr.port()).as_bytes()).await.expect("Failed to write proxy Protocol v1");
            }
            IpAddr::V6(_) => {
                todo!()
            }
        }
        Err(e) => println!("{}", e)
    }
    Ok(())
}

