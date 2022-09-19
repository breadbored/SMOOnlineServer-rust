mod packet;
mod server;
mod client;
use tokio::{
    net::{
        TcpSocket,
        TcpStream,
        TcpListener
    },
    io::{
        AsyncReadExt,
        AsyncWriteExt
    }
};
use std::{
    io::{prelude::*, BufReader, Result}, borrow::Borrow,
};

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "0.0.0.0:1027";

    let listener: TcpListener = TcpListener::bind(addr).await.unwrap();
    
    loop {
        let (mut socket, socket_addr) = listener.accept().await?;
        println!("new client: {:?}", socket_addr.to_string());

        tokio::spawn(async move {
            handle_socket(socket).await;
        });
    }
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