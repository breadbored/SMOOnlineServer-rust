mod packet;
mod server;
mod client;
mod constants;
mod settings;
use mempool::Pool;
use packet::packets::{IPacket, CapPacket::CapPacket};
use server::{Server, ServerWrapper};
use tokio::{
    net::TcpListener,
    sync::Mutex
};
use std::{io::Result, sync::{Arc}};

#[tokio::main]
async fn main() -> Result<()> {
    let addr: &str = "0.0.0.0:1027";
    let listener: TcpListener = TcpListener::bind(addr).await.unwrap();

    let server: Arc<Mutex<Server>> = Arc::new(
        Mutex::new(
            Server {
                listener: listener,
                clients: vec![],
                mempool: Pool::new(Box::new(|| [0; 1024])),
            }
        )
    );
    
    ServerWrapper::start(server).await?;

    Ok(())
}