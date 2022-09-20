mod packet;
mod server;
mod client;
use server::{Server, ServerTraits};
use tokio::net::TcpListener;
use std::io::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "0.0.0.0:1027";
    let listener: TcpListener = TcpListener::bind(addr).await.unwrap();

    let server = Server::new(listener);
    
    server.start().await?;
    Ok(())
}