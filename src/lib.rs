use tokio::net::TcpStream;
use protocol::v1::{proxy_protocolv1};
use protocol::v2::{proxy_protocolv2};

pub mod protocol;


pub async fn write_proxy(version: u16, client_stream: TcpStream, serveur_stream: &mut TcpStream) {

    if version == 1 {
        proxy_protocolv1(&client_stream, serveur_stream).await.expect("Failed to add Proxy Protocol v1");
    } else if version == 2 {
        proxy_protocolv2(&client_stream, serveur_stream).await.expect("Failed to add Proxy Protocol v2")
        
    }
}