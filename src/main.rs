mod packet;
mod server;
mod client;
mod constants;
mod settings;
use server::{Server, ServerWrapper};
use settings::Settings;
use tokio::{
    net::TcpListener,
    sync::{
        Mutex,
        RwLock,
    },
};
use std::{
    io::Result, sync::{
        Arc, 
    }
};

#[tokio::main]
async fn main() -> Result<()> {
    let settings = Settings::defaults();
    let addr: &str = "0.0.0.0:1027";
    let listener: TcpListener = TcpListener::bind(addr).await.unwrap();

    let server: Arc<Mutex<Server>> = Arc::new(
        Mutex::new(
            Server {
                clients: vec![],
                settings,
            }
        )
    );
    
    ServerWrapper::start(server, listener).await;

    Ok(())
}