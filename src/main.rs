mod packet;
mod server;
mod client;
mod constants;
mod settings;
mod lib;
use mempool::Pool;
use packet::packets::{IPacket, CapPacket::CapPacket};
use server::{Server, ServerWrapper};
use settings::{Settings};
use tokio::{
    net::TcpListener,
    sync::{Mutex, RwLock}
};
use std::{
    io::Result, sync::{
        Arc, 
        // Mutex
    }
};

#[tokio::main]
async fn main() -> Result<()> {
    let settings = Settings::defaults();
    let addr: &str = "0.0.0.0:1027";
    let listener: TcpListener = TcpListener::bind(addr).await.unwrap();

    let server: Arc<RwLock<Server>> = Arc::new(
        RwLock::new(
            Server {
                clients: vec![],
                mempool: Pool::new(Box::new(|| [0; 1024])),
                settings: settings,
            }
        )
    );
    
    ServerWrapper::start(server, listener).await;

    Ok(())
}