use std::{sync::{Arc}, f32::consts::PI, future::Future};
use nalgebra::{Vector3, Quaternion, Matrix4};
use tokio::{
    net::{
        TcpStream,
        TcpListener
    },
    io::{
        AsyncReadExt,
        Result
    },
    sync::Mutex
};
use chrono::{
    Utc
};
use mempool::Pool;
use uuid::Uuid;
use crate::{
    client::{
        Client,
        ClientTraits, Time
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
            },
            InitPacket::InitPacket,
            ConnectPacket::{ConnectPacket, ConnectionTypes},
            CostumePacket::CostumePacket,
            GamePacket::GamePacket, TagPacket::{TagPacket, TagUpdate}, PlayerPacket::PlayerPacket, CapPacket::CapPacket, UnhandledPacket::UnhandledPacket, DisconnectPacket::DisconnectPacket, ShinePacket::ShinePacket, CapturePacket::CapturePacket, ChangeStagePacket::ChangeStagePacket,
        }, PacketType::PacketType
    },
    constants::{
        packet_to_type_map
    },
    settings::{
        MAX_PLAYERS, Settings, FlipOptions,
    }
};

// Math shit
// I hate math
// However, nalgebra Quaternions and Matrices don't have these functions
trait QuaternionMatrixConvertible {
    fn create_from_rotation_matrix(matrix: Matrix4<f32>) -> Self;
    fn create_from_rotation_matrix_x() -> Self;
    fn create_from_rotation_matrix_y() -> Self;
}
impl QuaternionMatrixConvertible for Quaternion<f32> {
    fn create_from_rotation_matrix(matrix: Matrix4<f32>) -> Self {
        // println!("create_from_rotation_matrix");
        let w: f32 = (1.0 + matrix.m11 + matrix.m22 + matrix.m33).sqrt() / 2.0;
        let w4: f32 = 4.0 * w;
        let i: f32 = (matrix.m32 - matrix.m23) / w4;
        let j: f32 = (matrix.m13 - matrix.m31) / w4;
        let k: f32 = (matrix.m21 - matrix.m12) / w4;
        Quaternion::<f32> {
            w,
            i,
            j,
            k
        }
    }

    fn create_from_rotation_matrix_x() -> Self {
        // println!("create_from_rotation_matrix_x");
        let matrix = Matrix4::<f32>::create_rotation_x(PI);
        return Quaternion::<f32>::create_from_rotation_matrix(matrix);
    }

    fn create_from_rotation_matrix_y() -> Self {
        // println!("create_from_rotation_matrix_y");
        let matrix = Matrix4::<f32>::create_rotation_y(PI);
        return Quaternion::<f32>::create_from_rotation_matrix(matrix);
    }
}
trait MatrixConvertible {
    fn create_rotation_x(radians: f32) -> Self;
    fn create_rotation_y(radians: f32) -> Self;
}
impl MatrixConvertible for Matrix4<f32> {
    fn create_rotation_x(radians: f32) -> Self {
        // println!("create_rotation_x");
        Matrix4::<f32>::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, radians.cos(), -radians.sin(), 0.0,
            0.0, radians.sin(), radians.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0
        )
    }
    fn create_rotation_y(radians: f32) -> Self {
        // println!("create_rotation_y");
        Matrix4::<f32>::new(
            radians.cos(), 0.0, radians.sin(), 0.0,
            0.0, 1.0, 0.0, 0.0,
            -radians.sin(), 0.0, radians.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0
        )
    }
}

pub struct ServerWrapper {
    
}

pub struct Server {
    pub clients: Vec<Arc<Mutex<Client>>>,
    pub mempool: Pool<[u8; 1024]>,
    pub settings: Settings,
}

impl ServerWrapper {
    pub async fn start(server: Arc<Mutex<Server>>, listener: TcpListener) -> Result<()> {
        // println!("start");
        // Loop until new connection is made and spawn an async event loop
        loop {
            let (socket, socket_addr) = listener.accept().await?;
            println!("new client: {:?}", socket_addr.to_string());

            let local_server = server.clone();
            tokio::spawn(async move {
                ServerWrapper::handle_socket(local_server.clone(), socket).await
            });
        }
    }

    async fn handle_socket(server: Arc<Mutex<Server>>, socket_raw: TcpStream) {
        // println!("Handle Socket");
        let socket = Arc::new(
            Mutex::new(
                socket_raw
            )
        );
        let mut client = Arc::new(
            Mutex::new(
                Client::new(socket)
            )
        );
        
        let mut clients = server.lock().await.clients.clone();
        clients.push(client.clone());

        let mut first = true;

        // In a loop, read data from the socket and write the data back.
        let mut buffer: [u8; 1024] = [0; 1024];
        loop {
            // println!("loop");

            if first {
                // println!("First init packet");
                // Send Init packet to tell SMO Online it is connected
                let mut init_packet = IPacket::<InitPacket>::new();
                init_packet.packet.max_players = MAX_PLAYERS;

                let mut packet_header = IPacket::<PacketHeader>::new();
                packet_header.packet.id = client.lock().await.id;
                packet_header.packet.packet_size = init_packet.packet_size as u16;
                packet_header.packet.packet_type = PacketType::Init;

                println!("Are we stuck?");
                client.lock().await.send(
                    &packet_header,
                    &init_packet
                ).await;
                println!("Nope");
            }
            println!("{:?}", first);
            let n = (*client).lock().await.socket.lock().await
                .read(&mut buffer)
                .await
                .expect("failed to read data from socket");
            
            // println!("{:?}", n);
    
            if n == 0 {
                // println!("No Data");
                continue;
            }
    
            let incoming_buffer = &buffer[..n];

            let mut packet_header = IPacket::<PacketHeader>::new();
            packet_header.deserialize(&incoming_buffer[..packet_header.packet_size]);

            // packet_header.packet_size is the size of the header
            // packet_header.packet.packet_size is the size of the packet
            let packet_data = &incoming_buffer[packet_header.packet_size..(packet_header.packet_size + (packet_header.packet.packet_size as usize))];

            if first {
                first = false;

                let mut connect_packet = IPacket::<ConnectPacket>::new();
                connect_packet.deserialize(packet_data);

                if packet_header.packet.packet_type != PacketType::Connect {
                    break;
                }

                // todo: if too many clients connected, disconnect
                // https://github.com/Sanae6/SmoOnlineServer/blob/master/Server/Server.cs#L177-L181
                let mut connected_clients: Vec<&Arc<Mutex<Client>>> = Vec::new();
                let clients_iterable = clients.clone();
                for c_index in 0..clients_iterable.len() {
                    if clients_iterable[c_index].lock().await.connected {
                        connected_clients.push(&clients_iterable[c_index])
                    }
                }

                if connected_clients.len() >= MAX_PLAYERS.into() {
                    // println!("Disconnect: Too many players");
                    break;
                }

                let mut first_conn = false;

                match connect_packet.packet.connection_type {
                    ConnectionTypes::FirstConnection => {
                        first_conn = true;
                        let mut found_client = None;
                        let clients_iterable = connected_clients.clone();
                        for c_index in 0..clients_iterable.len() {
                            let local_client = clients_iterable[c_index].clone();
                            if local_client.lock().await.id == packet_header.packet.id {
                                found_client = Some(c_index);
                            }
                        }
                        match found_client {
                            Some(found_index) => {
                                if connected_clients[found_index].lock().await.connected {
                                    // println!("Disconnect already connected client")
                                    // TODO
                                }
                                client = connected_clients[found_index].clone();
                            },
                            None => {},
                        }
                    },
                    ConnectionTypes::Reconnecting => {
                        client.lock().await.id = packet_header.packet.id;
                        let mut found_client = None;
                        let clients_iterable = connected_clients.clone();
                        for c_index in 0..clients_iterable.len() {
                            let local_client = clients_iterable[c_index].clone();
                            if local_client.lock().await.id == packet_header.packet.id {
                                found_client = Some(c_index);
                            }
                        }
                        match found_client {
                            Some(found_index) => {
                                if connected_clients[found_index].lock().await.connected {
                                    // println!("Disconnect already connected client")
                                    // TODO
                                }
                                client = connected_clients[found_index].clone();
                            },
                            None => {
                                first_conn = true;
                                connect_packet.packet.connection_type = ConnectionTypes::FirstConnection;
                            },
                        }
                    },
                }

                client.lock().await.name = (*connect_packet.packet.client_name).to_string();
                client.lock().await.connected = true;
                if first_conn {
                    // let clients_iterable = clients.clone();
                    // for c_index in 0..clients_iterable.len() {
                    //     let local_client = clients_iterable[c_index].clone();
                    //     if local_client.lock().await.id != packet_header.packet.id {
                    //         found_client = Some(c_index);
                    //     }
                    // }
                    // server.lock().await.clients.retain(|f| f.blocking_lock().id != packet_header.packet.id);
                    client.lock().await.id = packet_header.packet.id;
                    server.lock().await.clients.push(client.clone());
                }

                let mut other_players: Vec<Arc<Mutex<Client>>> = Vec::new();
                let clients_iterable = clients.clone();
                for c_index in 0..clients_iterable.len() {
                    let local_client = clients_iterable[c_index].clone();
                    if local_client.lock().await.id != packet_header.packet.id {
                        other_players.push(local_client)
                    }
                }

                let local_connection_packet_size = *connect_packet.get_size() as u16;

                other_players.iter().for_each(|f| {
                    let player = f.clone();

                    tokio::spawn(async move {
                        let local_player = player.clone();
                        let mut temp_memory: [u8; 1024] = [0; 1024];
                        
                        let mut player_packet_header = IPacket::<PacketHeader>::new();
                        player_packet_header.packet.id = local_player.lock().await.id;
                        player_packet_header.packet.packet_type = PacketType::Connect;
                        player_packet_header.packet.packet_size = local_connection_packet_size;

                        temp_memory[..(player_packet_header.packet_size)].copy_from_slice(
                            &player_packet_header.serialize()[..(player_packet_header.packet_size)]
                        );

                        let mut player_packet_connection = IPacket::<ConnectPacket>::new();
                        player_packet_connection.packet.connection_type = ConnectionTypes::FirstConnection;
                        player_packet_connection.packet.max_players = MAX_PLAYERS;
                        player_packet_connection.packet.client_name = (*local_player.lock().await.name).to_string();

                        // This looks like shit. I'm just trying to copy data into a slice!
                        temp_memory[
                            (player_packet_header.packet_size)..(player_packet_header.packet_size + player_packet_connection.packet_size)
                        ].copy_from_slice(
                            &player_packet_header.serialize()[..(player_packet_connection.packet_size)]
                        );

                        local_player.lock().await.send_raw_data(
                            &temp_memory[..(player_packet_header.packet_size + player_packet_connection.packet_size)],
                            player_packet_header.packet_size + player_packet_connection.packet_size
                        ).await;
                    });
                });
            } else if packet_header.packet.id != client.lock().await.id && !Uuid::is_nil(&client.lock().await.id) {
                // println!("Invalid packet from {:#?}", client.lock().await.name);
            }

            if packet_header.packet.packet_type == PacketType::Costume {
                let mut costume_packet = IPacket::<CostumePacket>::new();
                costume_packet.deserialize(&incoming_buffer[packet_header.packet_size..(packet_header.packet_size + (packet_header.packet.packet_size as usize))]);
                client.lock().await.current_costume = Some(costume_packet);
            }

            // // println!("{:?}", incoming_buffer);

            match packet_header.packet.packet_type {
                PacketType::Cap => {
                    println!("Cap Packet:");
                    ServerWrapper::packet_builder::<IPacket::<CapPacket>>(
                        server.clone(),
                        client.clone(),
                        incoming_buffer.clone(),
                        &mut packet_header
                    ).await;
                },
                PacketType::Init => {
                    println!("Init Packet");
                    ServerWrapper::packet_builder::<IPacket::<InitPacket>>(
                        server.clone(),
                        client.clone(),
                        incoming_buffer.clone(),
                        &mut packet_header
                    ).await;
                },
                PacketType::Player => {
                    println!("Player Packet");
                    ServerWrapper::packet_builder::<IPacket::<PlayerPacket>>(
                        server.clone(),
                        client.clone(),
                        incoming_buffer.clone(),
                        &mut packet_header
                    ).await;
                },
                PacketType::Game => {
                    println!("Game Packet");
                    ServerWrapper::packet_builder::<IPacket::<GamePacket>>(
                        server.clone(),
                        client.clone(),
                        incoming_buffer.clone(),
                        &mut packet_header
                    ).await;
                },
                PacketType::Tag => {
                    println!("Tag Packet");
                    ServerWrapper::packet_builder::<IPacket::<TagPacket>>(
                        server.clone(),
                        client.clone(),
                        incoming_buffer.clone(),
                        &mut packet_header
                    ).await;
                },
                PacketType::Connect => {
                    println!("Connect Packet");
                    ServerWrapper::packet_builder::<IPacket::<ConnectPacket>>(
                        server.clone(),
                        client.clone(),
                        incoming_buffer.clone(),
                        &mut packet_header
                    ).await;
                },
                PacketType::Disconnect => {
                    println!("Disconnect Packet");
                    ServerWrapper::packet_builder::<IPacket::<DisconnectPacket>>(
                        server.clone(),
                        client.clone(),
                        incoming_buffer.clone(),
                        &mut packet_header
                    ).await;
                },
                PacketType::Costume => {
                    println!("Costume Packet");
                    ServerWrapper::packet_builder::<IPacket::<CostumePacket>>(
                        server.clone(),
                        client.clone(),
                        incoming_buffer.clone(),
                        &mut packet_header
                    ).await;
                },
                PacketType::Shine => {
                    println!("Shine Packet");
                    ServerWrapper::packet_builder::<IPacket::<ShinePacket>>(
                        server.clone(),
                        client.clone(),
                        incoming_buffer.clone(),
                        &mut packet_header
                    ).await;
                },
                PacketType::Capture => {
                    println!("Capture Packet");
                    ServerWrapper::packet_builder::<IPacket::<CapturePacket>>(
                        server.clone(),
                        client.clone(),
                        incoming_buffer.clone(),
                        &mut packet_header
                    ).await;
                },
                PacketType::ChangeStage => {
                    println!("ChangeStage Packet");
                    ServerWrapper::packet_builder::<IPacket::<ChangeStagePacket>>(
                        server.clone(),
                        client.clone(),
                        incoming_buffer.clone(),
                        &mut packet_header
                    ).await;
                },
                _ => {
                    // println!("Unknown Packet");
                    // let mut packet_serialized = IPacket::<UnhandledPacket>::new();
                    // packet_serialized.deserialize(&incoming_buffer[PACKET_HEADER_SIZE..(PACKET_HEADER_SIZE + packet_header.packet.packet_size as usize)]);
                    // ServerWrapper::packet_handler(server.clone(), client.clone(), packet_serialized).await;
                },
            }

            // client.lock().await.send(&mut socket, packet);
            
            // client.lock().await.send_raw_data(incoming_buffer, n).await;
        }
    }

    pub async fn packet_builder<T: IPacketTrait>(server: Arc<Mutex<Server>>, client: Arc<Mutex<Client>>, incoming_buffer: &[u8], packet_header: &mut IPacket<PacketHeader>) {
        let mut packet_serialized = T::new();
        packet_serialized.deserialize(&incoming_buffer[PACKET_HEADER_SIZE..(PACKET_HEADER_SIZE + packet_header.packet.packet_size as usize)]);
        let will_send = ServerWrapper::packet_handler(server.clone(), client.clone(), packet_header, packet_serialized).await;

        if will_send {
            // TODO: Implement Copy on packets so I don't need to copy and paste this so often
            let mut packet_serialized = T::new();
            packet_serialized.deserialize(&incoming_buffer[PACKET_HEADER_SIZE..(PACKET_HEADER_SIZE + packet_header.packet.packet_size as usize)]);
            ServerWrapper::broadcast(server, &mut packet_serialized, Some(client.clone())).await;
        }
    }

    pub fn fill_packet<T: IPacketTrait> (packet_header: &mut IPacket<PacketHeader>, packet: &mut T, memory: &Pool<[u8; 1024]>)
    where T: IPacketTrait
    {
        // println!("fill packet");
        let data: &[u8; 1024] = memory.get();
        
        packet_header.deserialize(&data[..PACKET_HEADER_SIZE]);
        packet.deserialize(&data[PACKET_HEADER_SIZE..(PACKET_HEADER_SIZE + packet_header.packet.packet_size as usize)]);
    }

    pub async fn broadcast_replace<T: IPacketTrait, Fut>(
        server: Arc<Mutex<Server>>,
        header: &mut IPacket<PacketHeader>,
        packet: T,
        client: Arc<Mutex<Client>>,
        packet_replacer: fn(
            server: Arc<Mutex<Server>>,
            from: Arc<Mutex<Client>>,
            to: Arc<Mutex<Client>>,
            header: &mut IPacket<PacketHeader>,
            p: &mut T
        ) -> Fut
    )
    where T: IPacketTrait,
            Fut: Future<Output = ()>
    {
        let clients_iterable = server.lock().await.clients.clone();
        for c_index in 0..clients_iterable.len() {
            let local_client = &clients_iterable[c_index];
            let local_server = server.clone();
            let mut packet_copy = T::new();
            packet_copy.deserialize(&packet.serialize()[..packet.get_size().to_owned()]);

            if local_client.lock().await.connected && client.lock().await.id != local_client.lock().await.id {
                packet_replacer(local_server.clone(), client.clone(), local_client.clone(), header, &mut packet_copy).await;
            }
        }
    }

    pub async fn broadcast<T: IPacketTrait>(server: Arc<Mutex<Server>>, packet: &mut T, client: Option<Arc<Mutex<Client>>>)
    where T: IPacketTrait
    {
        // println!("broadcast");
        let memory: &Pool<[u8; 1024]> = &server.lock().await.mempool;
        match client {
            Some(client) => {
                let mut copied_packet = T::new();
                copied_packet.deserialize(&packet.serialize()[..packet.get_size().to_owned()]);

                let packet_type_name = packet.get_name().clone();
                let packet_size_usize = packet.get_size().to_owned();
                let packet_type = packet_to_type_map(&packet_type_name);
                let packet_size = packet_size_usize as u16;
                
                let mut packet_header = IPacket::<PacketHeader>::new();
                packet_header.packet.id = client.lock().await.id;
                packet_header.packet.packet_type = packet_type;
                packet_header.packet.packet_size = packet_size;
                
                // Wait, what is this for again?
                // ServerWrapper::fill_packet::<T>(&mut packet_header, &mut copied_packet, memory);

                println!("Preparing to send from {:?}", client.lock().await.name);
                let clients = server.lock().await.clients.clone();
                for c in clients {
                    println!("Sending to {:?}", c.lock().await.name);
                    if c.lock().await.id != client.lock().await.id {
                        c.lock().await.send(&packet_header, &copied_packet).await;
                    }
                }
            },
            None => {

            }
        }
    }

    async fn packet_handler<T: IPacketTrait>(server: Arc<Mutex<Server>>, client: Arc<Mutex<Client>>, packet_header: &mut IPacket<PacketHeader>, packet: T) -> bool
    where T: IPacketTrait
    {
        // println!("packet_handler");
        match packet.get_name() {
            "GamePacket" => {
                let mut copied_packet = IPacket::<GamePacket>::new();
                copied_packet.deserialize(&packet.serialize()[..packet.get_size().to_owned()]);
                
                client.lock().await.metadata.scenario = copied_packet.packet.scenario_num;
                client.lock().await.metadata.is_2d = copied_packet.packet.is_2d;
                client.lock().await.metadata.last_game_packet = Some(copied_packet);

                // Recopy packet
                // TODO: Implement copy trait on IPacket<T> and all T
                copied_packet = IPacket::<GamePacket>::new();
                copied_packet.deserialize(&packet.serialize()[..packet.get_size().to_owned()]);

                match copied_packet.packet.stage.as_str() {
                    "CapWorldHomeStage" => {
                        client.lock().await.metadata.speedrun = true;

                        // Shine Sync
                        // https://github.com/Sanae6/SmoOnlineServer/blob/e14616030cea51d1508665d8c1e4743e9c70c290/Server/Program.cs#L128

                        // println!("Cap kingdom, do not sync shines");
                    },
                    "WaterfallWorldHomeStage" => {
                        let was_speedrun = client.lock().await.metadata.speedrun;
                        client.lock().await.metadata.speedrun = false;
                        if was_speedrun {
                            // Shine Sync with delay
                            // https://github.com/Sanae6/SmoOnlineServer/blob/e14616030cea51d1508665d8c1e4743e9c70c290/Server/Program.cs#L135-L140
                        }
                    },
                    _ => {

                    }
                }

                if server.lock().await.settings.scenario.merge_enabled {
                    ServerWrapper::broadcast_replace(
                        server.clone(),
                        packet_header,
                        copied_packet,
                        client.clone(),
                        |server: Arc<Mutex<Server>>, from: Arc<Mutex<Client>>, to: Arc<Mutex<Client>>, header: &mut IPacket<PacketHeader>, p: &mut IPacket<GamePacket>| {
                            let mut copied_packet_header = IPacket::<PacketHeader>::new();
                            copied_packet_header.deserialize(&header.serialize()[..header.get_size().to_owned()]);
                            let mut copied_packet = IPacket::<GamePacket>::new();
                            copied_packet.deserialize(&p.serialize()[..p.get_size().to_owned()]);
                            
                            async move {
                                copied_packet.packet.scenario_num = from.lock().await.metadata.scenario;

                                tokio::spawn(async move {
                                    to.lock().await.send(&copied_packet_header, &copied_packet).await;
                                });
                            }
                        }
                    ).await;
                    return false;
                }
            },
            "TagPacket" => {
                let mut copied_packet = IPacket::<TagPacket>::new();
                copied_packet.deserialize(&packet.serialize()[..packet.get_size().to_owned()]);

                if (copied_packet.packet.update_type as u8 & TagUpdate::State as u8) != 0 {
                    client.lock().await.metadata.seeking = copied_packet.packet.is_it;
                }
                if (copied_packet.packet.update_type as u8 & TagUpdate::Time as u8) != 0 {
                    client.lock().await.metadata.time = Time {
                        seconds: copied_packet.packet.seconds,
                        minutes: copied_packet.packet.minutes,
                        when: Utc::now(),
                    };
                }
            },
            "CostumePacket" => {
                // Shine sync
                // https://github.com/Sanae6/SmoOnlineServer/blob/e14616030cea51d1508665d8c1e4743e9c70c290/Server/Program.cs#L165
                client.lock().await.metadata.loaded_save = true;
            },
            "ShinePacket" => {
                if client.lock().await.metadata.loaded_save {
                    // Shine sync
                    // https://github.com/Sanae6/SmoOnlineServer/blob/e14616030cea51d1508665d8c1e4743e9c70c290/Server/Program.cs#L169-L178
                }
            },
            "PlayerPacket" => {
                let mut player_packet = IPacket::<PlayerPacket>::new();
                player_packet.deserialize(&packet.serialize()[..packet.get_size().to_owned()]);

                if server.lock().await.settings.flip.enabled &&
                    (server.lock().await.settings.flip.pov == FlipOptions::BothOption || server.lock().await.settings.flip.pov == FlipOptions::SelfOption) &&
                    server.lock().await.settings.flip.players.contains(&client.lock().await.id)
                {
                    player_packet.packet.position = Vector3::<f32>::new(0.0, 1.0, 0.0) * ServerWrapper::mario_size(client.lock().await.metadata.is_2d);
                    player_packet.packet.rotation *= Quaternion::<f32>::create_from_rotation_matrix_x() * Quaternion::<f32>::create_from_rotation_matrix_y();
                    ServerWrapper::broadcast::<IPacket<PlayerPacket>>(server.clone(), &mut player_packet, Some(client)).await;
                } else if server.lock().await.settings.flip.enabled &&
                    (server.lock().await.settings.flip.pov == FlipOptions::BothOption || server.lock().await.settings.flip.pov == FlipOptions::OthersOption) &&
                    server.lock().await.settings.flip.players.contains(&client.lock().await.id)
                {
                    player_packet.packet.position = Vector3::<f32>::new(0.0, 1.0, 0.0) * ServerWrapper::mario_size(client.lock().await.metadata.is_2d);
                    player_packet.packet.rotation *= Quaternion::<f32>::create_from_rotation_matrix_x() * Quaternion::<f32>::create_from_rotation_matrix_y();
                    ServerWrapper::broadcast_replace(
                        server.clone(),
                        packet_header,
                        player_packet,
                        client.clone(),
                        |server: Arc<Mutex<Server>>, from: Arc<Mutex<Client>>, to: Arc<Mutex<Client>>, header: &mut IPacket<PacketHeader>, p: &mut IPacket<PlayerPacket>| {
                            let mut copied_packet_header = IPacket::<PacketHeader>::new();
                            copied_packet_header.deserialize(&header.serialize()[..header.get_size().to_owned()]);
                            let mut copied_packet = IPacket::<PlayerPacket>::new();
                            copied_packet.deserialize(&p.serialize()[..p.get_size().to_owned()]);

                            async move {
                                if server.lock().await.settings.flip.players.contains(&to.lock().await.id) {
                                    copied_packet.packet.position = Vector3::<f32>::new(0.0, 1.0, 0.0) * ServerWrapper::mario_size(from.lock().await.metadata.is_2d);
                                    copied_packet.packet.rotation *= Quaternion::<f32>::create_from_rotation_matrix_x() * Quaternion::<f32>::create_from_rotation_matrix_y();
                                }

                                tokio::spawn(async move {
                                    to.lock().await.send(&copied_packet_header, &copied_packet).await;
                                });
                            }
                        }
                    ).await;
                    return false;
                }
            },
            _ => {
                // println!("Unsupported Packet?");
            }
        }

        return true;
    }

    fn mario_size(is_2d: bool) -> f32 {
        // println!("mario_size");
        if is_2d {
            return 180.0;
        }
        return 160.0;
    }
}