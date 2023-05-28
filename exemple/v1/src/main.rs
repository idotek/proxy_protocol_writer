use tokio::{net::{TcpListener, TcpStream}};

use proxyProtocol::protocol::v2::{proxy_protocolv2};
use proxyProtocol::protocol::v1::{proxy_protocolv1};

use anyhow::{anyhow, Result};
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.expect("Failed to start Listener.");
    loop {
        match listener.accept().await {
            Ok((client_stream, addr)) => {         
                tokio::spawn(async move {
                    match TcpStream::connect("127.0.0.1:8081").await {
                        Ok(mut backend_stream) => {
                            proxy_protocolv1(&client_stream, &mut backend_stream).await.expect("Failed to add proxy protocol v2");
                            reverse(client_stream, backend_stream).await.expect("Failed to send packet to backend side");
                        }
                        Err(e) => println!("{}", e)
                    }
                });
            }
            Err(e) => println!("{}", e)
        }
    }
}



async fn reverse(mut client_stream: TcpStream, mut serveur_stream: TcpStream) -> Result<()> {

    tokio::io::copy_bidirectional(&mut client_stream, &mut serveur_stream).await.map_err(|e| anyhow!("Cant send packet {}", e))?;


    Ok(())

}
