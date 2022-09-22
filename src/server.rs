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
    client::{
        Client,
        ClientTraits
    },
    packet::{
        PacketHeader::{
            PacketHeader,
            SIZE as PACKET_HEADER_SIZE
        },
        packets::{
            IPacket::{
                IPacket,
                IPacketTrait,
            }, InitPacket::InitPacket,
        }
    },
    constants::{
        packet_to_type_map
    },
    settings::{
        MAX_PLAYERS
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
        let client = Arc::new(
            Mutex::new(
                Client::new(&server)
            )
        );
        server.lock().await.clients.push(client);

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
    
            let mut init_packet = IPacket::<InitPacket>::new();
            init_packet.packet.max_players = MAX_PLAYERS;

            // client.lock().await.send::<, IPacket<InitPacket>>(
            //     socket, 
            //     init_packet
            // );
    
            socket
                .write_all(temp_buffer)
                .await
                .expect("failed to write data to socket");
        }
    }

    pub fn fill_packet<T: IPacketTrait> (packet_header: &mut IPacket<PacketHeader>, packet: &mut T, memory: Pool<[u8; 1024]>)
    where T: Copy + IPacketTrait
    {
        let data: &[u8; 1024] = memory.get();
        
        packet_header.deserialize(&data[..PACKET_HEADER_SIZE]);
        packet.deserialize(&data[PACKET_HEADER_SIZE..]);
    }

    pub async fn broadcast<T: IPacketTrait>(server: Server, packet: &mut T, client: Option<Client>)
    where T: Copy + IPacketTrait
    {
        let memory: Pool<[u8; 1024]> = server.mempool;
        match client {
            Some(c) => {
                let packet_type_name = packet.get_name().clone();
                let packet_size_usize = packet.get_size().to_owned();
                let packet_type = packet_to_type_map(&packet_type_name);
                let packet_size = packet_size_usize as i16;
                
                let mut packet_header = IPacket::<PacketHeader>::new();
                packet_header.packet.id = c.id;
                packet_header.packet.packet_type = packet_type;
                packet_header.packet.packet_size = packet_size;
                
                ServerWrapper::fill_packet::<T>(&mut packet_header, packet, memory);
            },
            None => {

            }
        }
    }
}