use tokio::{
    net::{
        TcpStream,
        TcpListener
    },
    io::{
        AsyncReadExt,
        AsyncWriteExt,
        Result
    }
};
use mempool::Pool;
use crate::client::Client;

pub struct Server<'a> {
    pub listener: TcpListener,
    pub clients: Vec<Client<'a>>,
    pub mempool: Pool<[u8; 1024]>,
}

pub trait ServerTraits {
    fn new(listener: TcpListener) -> Self;
}

impl ServerTraits for Server<'_> {
    fn new(listener: TcpListener) -> Self {
        Server {
            listener: listener,
            clients: vec![],
            mempool: Pool::new(Box::new(|| [0; 1024])),
        }
    }
}
impl Server<'_> {
    pub async fn start(&self) -> Result<()> {
        // Loop until new connection is made and spawn an async event loop
        loop {
            let (mut socket, socket_addr) = self.listener.accept().await?;
            println!("new client: {:?}", socket_addr.to_string());

            tokio::spawn(async move {
                Server::handle_socket(socket).await;
            });

            // Trick the compiler into thinking this eventually responds with Okay(())
            if false {
                break
            };
        }
        return Ok(())
    }

    async fn handle_socket(mut socket: TcpStream) {
        let mut buffer: [u8; 128] = [0; 128];
        // In a loop, read data from the socket and write the data back.
        loop {
            let n = socket
                .read(&mut buffer)
                .await
                .expect("failed to read data from socket");
    
            if n == 0 {
                continue;
            }
    
            let mut temp_buffer = &buffer[0..n];
    
            print!("{:?}", temp_buffer);
    
            socket
                .write_all(temp_buffer)
                .await
                .expect("failed to write data to socket");
        }
    }
}