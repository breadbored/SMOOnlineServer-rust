use std::sync::{Arc};
use tokio::{
    net::{
        TcpStream,
        TcpListener
    },
    io::{
        AsyncReadExt,
        AsyncWriteExt,
        Result
    },
    sync::Mutex
};
use mempool::Pool;
use crate::{
    client::Client,
    packet::{
        PacketHeader::{
            PacketHeader,
            SIZE as PACKET_HEADER_SIZE
        },
        packets::{
            IPacket::{
                IPacket,
                IPacketTrait,
            },
            CapPacket::CapPacket,
        }
    }
};

pub struct ServerWrapper {
    
}

pub struct Server {
    pub listener: TcpListener,
    pub clients: Vec<Arc<Mutex<Client>>>,
    pub mempool: Pool<[u8; 1024]>,
}

impl ServerWrapper {
    pub async fn start(server: Arc<Mutex<Server>>) -> Result<()> {
        // Loop until new connection is made and spawn an async event loop
        loop {
            let (mut socket, socket_addr) = server.lock().await.listener.accept().await?;
            println!("new client: {:?}", socket_addr.to_string());

            let local_server = server.clone();
            tokio::spawn(ServerWrapper::handle_socket(local_server, socket));

            // Trick the compiler into thinking this eventually responds with Okay(())
            if false {
                break
            };
        }

        return Ok(())
    }

    async fn handle_socket(server: Arc<Mutex<Server>>, mut socket: TcpStream) {
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
    
            // print!("{:?}\n", temp_buffer);
    
            socket
                .write_all(temp_buffer)
                .await
                .expect("failed to write data to socket");
        }
    }

    pub fn fill_packet<T: IPacketTrait> (packet_header: &mut IPacket<PacketHeader>, packet: &mut T, memory: Pool<[u8; 1024]>)
    {
        let data: &[u8; 1024] = memory.get();
        
        packet_header.deserialize(&data[..PACKET_HEADER_SIZE]);
        packet.deserialize(&data[PACKET_HEADER_SIZE..]);
    }
}