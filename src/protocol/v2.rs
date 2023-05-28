use std::net::IpAddr;
use anyhow::{Result};
use tokio::{net::{TcpStream}};
use tokio::io::{AsyncWriteExt};

pub async fn proxy_protocolv2(client_stream: &TcpStream, serveur_stream: &mut TcpStream) -> Result<()> {
    let streamaddr = client_stream.local_addr().expect("Failed to get local_addr");
    match client_stream.peer_addr() {
        Ok(peeraddr) => {
          
          let mut cursor: Vec<u8> = Vec::new();
            
            cursor.write_all(b"\x0D\x0A\x0D\x0A\x00\x0D\x0A\x51\x55\x49\x54\x0A").await.expect("Failed to add signature");
            cursor.write_all(b"\x21").await.expect("Failed to add PROXY version");
            cursor.write_all(b"\x11").await.expect("Failed to add protocol ip version");
            cursor.write_all(&12u16.to_be_bytes()).await.expect("Failed to add packet len");
    
            match peeraddr.ip() {
                IpAddr::V4(ip4) => {
                    cursor.write_all(&ip4.octets()).await.expect("Failed to add source addr")
    
                }
                IpAddr::V6(_ip6) => {
                 todo!()   
                }
            }
    
            match streamaddr.ip() {
                IpAddr::V4(ip4) => {
                    cursor.write_all(&ip4.octets()).await.expect("Failed to write dst ip addr");
                }
                IpAddr::V6(_) => {
                    todo!()
                }
            }
    
            cursor.write_all(&peeraddr.port().to_be_bytes()).await.expect("msg");
            cursor.write_all(&streamaddr.port().to_be_bytes()).await.expect("Failed to add dst port");
            
    
            serveur_stream.write_all(&cursor).await.expect("Failed to add proxy protocol");
        }
        Err(e) => println!("{}", e)
    }
    Ok(())
}