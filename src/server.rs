use std::{
    sync::{
        Arc,
    }, 
    future::Future
};
use nalgebra::{Vector3, Quaternion};
use tokio::{
    net::{
        TcpStream,
        TcpListener
    },
    io::{
        AsyncReadExt,
        Result
    },
    sync::{Mutex, RwLock}
};
use chrono::{
    Utc
};
use mempool::Pool;
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
            ConnectPacket::{
                ConnectPacket,
                ConnectionTypes
            },
            CostumePacket::CostumePacket,
            GamePacket::GamePacket,
            TagPacket::{
                TagPacket, 
                TagUpdate
            }, 
            PlayerPacket::PlayerPacket, 
            CapPacket::CapPacket, 
            // UnhandledPacket::UnhandledPacket, 
            DisconnectPacket::DisconnectPacket, 
            ShinePacket::ShinePacket, 
            CapturePacket::CapturePacket, 
            ChangeStagePacket::ChangeStagePacket,
        }, 
        PacketType::PacketType
    },
    constants::{
        packet_to_type_map, type_to_packet_map
    },
    settings::{
        MAX_PLAYERS, 
        Settings, 
        FlipOptions,
    }, 
    lib::rot::QuaternionMatrixConvertible
};

pub struct ServerWrapper {
    pub server: Arc<RwLock<Server>>
}

pub struct Server {
    pub clients: Vec<Arc<RwLock<Client>>>,
    pub mempool: Pool<[u8; 1024]>,
    pub settings: Settings,
}

impl ServerWrapper {
    pub async fn start(server: Arc<RwLock<Server>>, listener: TcpListener) -> Result<()> {
        println!("start");
        // Loop until new connection is made and spawn an async event loop
        loop {
            let (socket, socket_addr) = listener.accept().await?;
            println!("new client: {:?}", socket_addr.to_string());

            let local_server = server.clone();
            tokio::spawn(async move {
                // ServerWrapper::handle_socket(local_server.clone(), socket).await
                ServerWrapper::handle_request(local_server.clone(), socket).await
            });
        }
    }

    async fn handle_request(server: Arc<RwLock<Server>>, socket: TcpStream) {
        println!("handle_request");
        let mut first_connection = true;

        let socket_mutex = Arc::new(
            Mutex::new(
                socket
            )
        );
        let client = Arc::new(
            RwLock::new(
                Client::new(socket_mutex)
            )
        );

        let mut init_packet = IPacket::<InitPacket>::new();
        init_packet.packet.max_players = MAX_PLAYERS;
        let mut init_packet_header = IPacket::<PacketHeader>::new();
        init_packet_header.packet.packet_size = init_packet.packet_size as i16;
        init_packet_header.packet.packet_type = PacketType::Init;
        client.read().await.send(&init_packet_header, &init_packet).await;

        loop {
            let mut buffer: [u8; 1024] = [0; 1024];
            let bytes_result = client.read().await.socket.lock().await
                .read(&mut buffer)
                .await;
            
            println!("-------------------- NEW PACKET --------------------");
            
            match bytes_result {
                Ok(num_bytes) => {
                    if num_bytes == 0 {
                        continue;
                    }

                    let incoming_buffer = &buffer[..num_bytes];

                    let mut packet_header = IPacket::<PacketHeader>::new();
                    packet_header.deserialize(&incoming_buffer[..packet_header.packet_size]);

                    // Handle UnhandledPackets up here
                    // They have a packet size of 0 in the header and break later logic
                    if type_to_packet_map(packet_header.packet.packet_type) == "UnhandledPacket" {
                        println!("Unhandled Packet");

                        // Should it send?
                        ServerWrapper::broadcast_raw(
                            server.clone(),
                            &buffer,
                            num_bytes,
                            client.clone()
                        ).await;

                        continue;
                    }

                    if packet_header.packet_size > 1024 || packet_header.packet.packet_size > 1024 || packet_header.packet.packet_size as usize > 1024 {
                        println!("Packet Type: {:?}", type_to_packet_map(packet_header.packet.packet_type));
                        println!("Header Size: {:?}", packet_header.packet_size);
                        println!("Packet Size: {:?}", packet_header.packet.packet_size);
                    }

                    let packet_data = &incoming_buffer[packet_header.packet_size..];

                    if first_connection || client.read().await.id != packet_header.packet.id {
                        client.write().await.id = packet_header.packet.id;

                        // Handle init to add or replace in client list
                        let mut connect_packet = IPacket::<ConnectPacket>::new();
                        connect_packet.deserialize(packet_data);
                        
                        match connect_packet.packet.connection_type {
                            ConnectionTypes::FirstConnection | ConnectionTypes::Reconnecting => {
                                println!("First connection / reconnect");

                                client.write().await.name = connect_packet.packet.client_name;
                                client.write().await.connected = true;

                                println!("Welcome, {:?}", client.read().await.name);

                                let mut already_connected: bool = false;
                                let mut connected_index: usize = 0;
                                for i in  0..server.read().await.clients.len() {
                                    if server.read().await.clients[i].read().await.id == client.read().await.id {
                                        already_connected = true;
                                        connected_index = i;
                                        break;
                                    }
                                }

                                if already_connected {
                                    server.write().await.clients[connected_index] = client.clone();
                                } else {
                                    server.write().await.clients.push(client.clone());
                                }

                                first_connection = false;

                                let mut local_connect_packet = IPacket::<ConnectPacket>::new();
                                local_connect_packet.deserialize(packet_data);
                                ServerWrapper::broadcast(server.clone(), &mut local_connect_packet, client.clone()).await;
                            },
                        }

                        if connect_packet.packet.connection_type == ConnectionTypes::FirstConnection {
                            ServerWrapper::sync_connect(server.clone(), client.clone()).await;
                        }
                    }

                    // Set the client's Costume
                    // Typically after connecting the user has to die to get this to show to other clients
                    if packet_header.packet.packet_type == PacketType::Costume {
                        let mut costume_packet = IPacket::<CostumePacket>::new();
                        costume_packet.deserialize(&incoming_buffer[packet_header.packet_size..]);
                        client.write().await.current_costume = Some(costume_packet.copy());
                    }
                    
                    println!("{:?}", type_to_packet_map(packet_header.packet.packet_type));

                    match packet_header.packet.packet_type {
                        PacketType::Cap => {
                            ServerWrapper::packet_builder::<IPacket::<CapPacket>>(
                                server.clone(),
                                client.clone(),
                                incoming_buffer.clone(),
                                &mut packet_header
                            ).await;
                        },
                        PacketType::Init => {
                            ServerWrapper::packet_builder::<IPacket::<InitPacket>>(
                                server.clone(),
                                client.clone(),
                                incoming_buffer.clone(),
                                &mut packet_header
                            ).await;
                        },
                        PacketType::Player => {
                            ServerWrapper::packet_builder::<IPacket::<PlayerPacket>>(
                                server.clone(),
                                client.clone(),
                                incoming_buffer.clone(),
                                &mut packet_header
                            ).await;
                        },
                        PacketType::Game => {
                            ServerWrapper::packet_builder::<IPacket::<GamePacket>>(
                                server.clone(),
                                client.clone(),
                                incoming_buffer.clone(),
                                &mut packet_header
                            ).await;
                        },
                        PacketType::Tag => {
                            ServerWrapper::packet_builder::<IPacket::<TagPacket>>(
                                server.clone(),
                                client.clone(),
                                incoming_buffer.clone(),
                                &mut packet_header
                            ).await;
                        },
                        PacketType::Connect => {
                            ServerWrapper::packet_builder::<IPacket::<ConnectPacket>>(
                                server.clone(),
                                client.clone(),
                                incoming_buffer.clone(),
                                &mut packet_header
                            ).await;
                        },
                        PacketType::Disconnect => {
                            ServerWrapper::packet_builder::<IPacket::<DisconnectPacket>>(
                                server.clone(),
                                client.clone(),
                                incoming_buffer.clone(),
                                &mut packet_header
                            ).await;
                        },
                        PacketType::Costume => {
                            ServerWrapper::packet_builder::<IPacket::<CostumePacket>>(
                                server.clone(),
                                client.clone(),
                                incoming_buffer.clone(),
                                &mut packet_header
                            ).await;
                        },
                        PacketType::Shine => {
                            ServerWrapper::packet_builder::<IPacket::<ShinePacket>>(
                                server.clone(),
                                client.clone(),
                                incoming_buffer.clone(),
                                &mut packet_header
                            ).await;
                        },
                        PacketType::Capture => {
                            ServerWrapper::packet_builder::<IPacket::<CapturePacket>>(
                                server.clone(),
                                client.clone(),
                                incoming_buffer.clone(),
                                &mut packet_header
                            ).await;
                        },
                        PacketType::ChangeStage => {
                            ServerWrapper::packet_builder::<IPacket::<ChangeStagePacket>>(
                                server.clone(),
                                client.clone(),
                                incoming_buffer.clone(),
                                &mut packet_header
                            ).await;
                        },
                        _ => {
                            println!("Unknown Packet");
                            // let mut packet_serialized = IPacket::<UnhandledPacket>::new();
                            // packet_serialized.deserialize(&incoming_buffer[PACKET_HEADER_SIZE..(PACKET_HEADER_SIZE + packet_header.packet.packet_size as usize)]);
                            // ServerWrapper::packet_handler(server.clone(), client.clone(), packet_serialized).await;
                        },
                    }
                },
                _ => {
                    client.write().await.connected = false;
                    return;
                }
            }
        }
    }



    pub async fn packet_builder<T: IPacketTrait>(server: Arc<RwLock<Server>>, client: Arc<RwLock<Client>>, incoming_buffer: &[u8], packet_header: &mut IPacket<PacketHeader>) {
        println!("packet_builder");
        let mut packet_serialized = T::new();
        packet_serialized.deserialize(&incoming_buffer[PACKET_HEADER_SIZE..(PACKET_HEADER_SIZE + packet_header.packet.packet_size as usize)]);
        let will_send = ServerWrapper::packet_handler(server.clone(), client.clone(), packet_header, packet_serialized).await;

        if will_send {
            // TODO: Implement Copy on packets so I don't need to copy and paste this so often
            let mut packet_serialized = T::new();
            packet_serialized.deserialize(&incoming_buffer[PACKET_HEADER_SIZE..(PACKET_HEADER_SIZE + packet_header.packet.packet_size as usize)]);
            ServerWrapper::broadcast(server.clone(), &mut packet_serialized, client.clone()).await;
        }
    }

    async fn packet_handler<T: IPacketTrait>(server: Arc<RwLock<Server>>, client: Arc<RwLock<Client>>, packet_header: &mut IPacket<PacketHeader>, packet: T) -> bool
    where T: IPacketTrait
    {
        println!("packet_handler");
        match packet.get_name() {
            "GamePacket" => {
                let mut copied_packet = IPacket::<GamePacket>::new();
                copied_packet.deserialize(&packet.serialize()[..packet.get_size().to_owned()]);
                
                client.write().await.metadata.scenario = copied_packet.packet.scenario_num;
                client.write().await.metadata.is_2d = copied_packet.packet.is_2d;
                client.write().await.metadata.last_game_packet = Some(copied_packet);

                // Recopy packet
                copied_packet = IPacket::<GamePacket>::new();
                copied_packet.deserialize(&packet.serialize()[..packet.get_size().to_owned()]);

                match copied_packet.packet.stage.as_str() {
                    "CapWorldHomeStage" => {
                        client.write().await.metadata.speedrun = true;

                        // Shine Sync
                        // https://github.com/Sanae6/SmoOnlineServer/blob/e14616030cea51d1508665d8c1e4743e9c70c290/Server/Program.cs#L128

                        // println!("Cap kingdom, do not sync shines");
                    },
                    "WaterfallWorldHomeStage" => {
                        let was_speedrun = client.read().await.metadata.speedrun;
                        client.write().await.metadata.speedrun = false;
                        if was_speedrun {
                            // Shine Sync with delay
                            // https://github.com/Sanae6/SmoOnlineServer/blob/e14616030cea51d1508665d8c1e4743e9c70c290/Server/Program.cs#L135-L140
                        }
                    },
                    _ => {

                    }
                }

                if server.read().await.settings.scenario.merge_enabled {
                    ServerWrapper::broadcast_replace(
                        server.clone(),
                        packet_header,
                        copied_packet,
                        client.clone(),
                        |_server: Arc<RwLock<Server>>, from: Arc<RwLock<Client>>, to: Arc<RwLock<Client>>, header: &mut IPacket<PacketHeader>, p: &mut IPacket<GamePacket>| {
                            println!("anonymous 2");
                            let mut copied_packet_header = IPacket::<PacketHeader>::new();
                            copied_packet_header.deserialize(&header.serialize()[..header.get_size().to_owned()]);
                            let mut copied_packet = IPacket::<GamePacket>::new();
                            copied_packet.deserialize(&p.serialize()[..p.get_size().to_owned()]);
                            
                            return async move {
                                copied_packet.packet.scenario_num = from.read().await.metadata.scenario;

                                let result = to.read().await.send(&copied_packet_header, &copied_packet).await;
                                if !result {
                                    to.write().await.connected = false;
                                }
                            };
                        }
                    ).await;
                    return false;
                }
            },
            "TagPacket" => {
                let mut copied_packet = IPacket::<TagPacket>::new();
                copied_packet.deserialize(&packet.serialize()[..packet.get_size().to_owned()]);

                if (copied_packet.packet.update_type as u8 & TagUpdate::State as u8) != 0 {
                    client.write().await.metadata.seeking = copied_packet.packet.is_it;
                }
                if (copied_packet.packet.update_type as u8 & TagUpdate::Time as u8) != 0 {
                    client.write().await.metadata.time = Time {
                        seconds: copied_packet.packet.seconds,
                        minutes: copied_packet.packet.minutes,
                        when: Utc::now(),
                    };
                }
            },
            "CostumePacket" => {
                // Shine sync
                // https://github.com/Sanae6/SmoOnlineServer/blob/e14616030cea51d1508665d8c1e4743e9c70c290/Server/Program.cs#L165
                client.write().await.metadata.loaded_save = true;
            },
            "ShinePacket" => {
                if client.write().await.metadata.loaded_save {
                    // Shine sync
                    // https://github.com/Sanae6/SmoOnlineServer/blob/e14616030cea51d1508665d8c1e4743e9c70c290/Server/Program.cs#L169-L178
                }
            },
            "PlayerPacket" => {
                let mut player_packet = IPacket::<PlayerPacket>::new();
                player_packet.deserialize(&packet.serialize()[..packet.get_size().to_owned()]);

                let settings = &server.read().await.settings;
                if settings.flip.enabled &&
                    (settings.flip.pov == FlipOptions::BothOption || settings.flip.pov == FlipOptions::SelfOption) &&
                    settings.flip.players.contains(&client.read().await.id)
                {
                    player_packet.packet.position = Vector3::<f32>::new(0.0, 1.0, 0.0) * ServerWrapper::mario_size(client.read().await.metadata.is_2d);
                    player_packet.packet.rotation *= Quaternion::<f32>::create_from_rotation_matrix_x() * Quaternion::<f32>::create_from_rotation_matrix_y();
                    ServerWrapper::broadcast::<IPacket<PlayerPacket>>(server.clone(), &mut player_packet, client.clone()).await;
                } else if settings.flip.enabled &&
                    (settings.flip.pov == FlipOptions::BothOption || settings.flip.pov == FlipOptions::OthersOption) &&
                    settings.flip.players.contains(&client.read().await.id)
                {
                    player_packet.packet.position = Vector3::<f32>::new(0.0, 1.0, 0.0) * ServerWrapper::mario_size(client.read().await.metadata.is_2d);
                    player_packet.packet.rotation *= Quaternion::<f32>::create_from_rotation_matrix_x() * Quaternion::<f32>::create_from_rotation_matrix_y();
                    ServerWrapper::broadcast_replace(
                        server.clone(),
                        packet_header,
                        player_packet,
                        client.clone(),
                        |server: Arc<RwLock<Server>>, from: Arc<RwLock<Client>>, to: Arc<RwLock<Client>>, header: &mut IPacket<PacketHeader>, p: &mut IPacket<PlayerPacket>| {
                            println!("anonymous 1");
                            let mut copied_packet_header = IPacket::<PacketHeader>::new();
                            copied_packet_header.deserialize(&header.serialize()[..header.get_size().to_owned()]);
                            let mut copied_packet = IPacket::<PlayerPacket>::new();
                            copied_packet.deserialize(&p.serialize()[..p.get_size().to_owned()]);

                            return async move {
                                if server.read().await.settings.flip.players.contains(&to.read().await.id) {
                                    copied_packet.packet.position = Vector3::<f32>::new(0.0, 1.0, 0.0) * ServerWrapper::mario_size(from.read().await.metadata.is_2d);
                                    copied_packet.packet.rotation *= Quaternion::<f32>::create_from_rotation_matrix_x() * Quaternion::<f32>::create_from_rotation_matrix_y();
                                }

                                let result = to.read().await.send(&copied_packet_header, &copied_packet).await;
                                if !result {
                                    to.write().await.connected = false;
                                }
                            };
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

    pub async fn broadcast_replace<T: IPacketTrait, Fut>(
        server: Arc<RwLock<Server>>,
        header: &mut IPacket<PacketHeader>,
        packet: T,
        client: Arc<RwLock<Client>>,
        packet_replacer: fn(
            server: Arc<RwLock<Server>>,
            from: Arc<RwLock<Client>>,
            to: Arc<RwLock<Client>>,
            header: &mut IPacket<PacketHeader>,
            p: &mut T
        ) -> Fut
    )
    where T: IPacketTrait,
            Fut: Future<Output = ()>
    {
        println!("broadcast_replace");
        let clients_iterable = &server.read().await.clients;
        for c_index in 0..clients_iterable.len() {
            let local_client = &clients_iterable[c_index];
            let local_server = server.clone();
            let mut packet_copy = T::new();
            packet_copy.deserialize(&packet.serialize()[..packet.get_size().to_owned()]);

            if local_client.read().await.connected && client.read().await.id != local_client.read().await.id {
                packet_replacer(local_server, client.clone(), local_client.clone(), header, &mut packet_copy).await;
            }
        }
    }

    pub async fn sync_connect(server: Arc<RwLock<Server>>, client: Arc<RwLock<Client>>)
    {
        println!("sync_connect");

        println!("Async To Player: {:?}", client.read().await.name);

        let client_id = client.read().await.id;
        let all_players = &server.read().await.clients;
        let mut other_players: Vec<Arc<RwLock<Client>>> = Vec::new();
        let num_players = all_players.len();
        println!("Num Players: {:?}", num_players);
        for i in 0..num_players {
            let player = all_players[i].clone();
            println!("Testing player if it is the client {:?}, {:?}", i, player.read().await.name);
            if player.read().await.id != client_id {
                println!("Testing player if it is connected {:?}, {:?}", i, player.read().await.name);
                if player.read().await.connected {
                    println!("Adding player {:?}, {:?}", i, player.read().await.name);
                    other_players.push(player.clone());
                }
            }
        }
        for c in other_players {
            println!("Sync From Player: {:?}", c.read().await.name);

            let mut connect_packet = IPacket::<ConnectPacket>::new();
            connect_packet.packet.client_name = c.read().await.name.as_str().to_string();
            connect_packet.packet.connection_type = ConnectionTypes::FirstConnection;
            connect_packet.packet.max_players = MAX_PLAYERS;

            let packet_type_name = connect_packet.get_name().clone();
            let packet_size_usize = connect_packet.get_size().to_owned();
            let packet_type = packet_to_type_map(&packet_type_name);
            let packet_size = packet_size_usize as i16;
            
            let mut packet_header = IPacket::<PacketHeader>::new();
            packet_header.packet.id = client.read().await.id;
            packet_header.packet.packet_type = packet_type;
            packet_header.packet.packet_size = packet_size;

            let result = client.read().await.send(&packet_header, &connect_packet).await;
            if !result {
                println!("{:?} disconnected due to client sync", c.read().await.name);
                client.write().await.connected = false;
            }
        }
        println!("Sent sync packets");
    }

    pub async fn broadcast<T: IPacketTrait>(server: Arc<RwLock<Server>>, packet: &mut T, client: Arc<RwLock<Client>>)
    where T: IPacketTrait
    {
        println!("broadcast");
        let mut copied_packet = T::new();
        copied_packet.deserialize(&packet.serialize()[..packet.get_size().to_owned()]);

        let packet_type_name = packet.get_name().clone();
        let packet_size_usize = packet.get_size().to_owned();
        let packet_type = packet_to_type_map(&packet_type_name);
        let packet_size = packet_size_usize as i16;
        
        let mut packet_header = IPacket::<PacketHeader>::new();
        packet_header.packet.id = client.read().await.id;
        packet_header.packet.packet_type = packet_type;
        packet_header.packet.packet_size = packet_size;

        println!("From Player: {:?}", client.read().await.name);

        let client_id = client.read().await.id;
        let all_players = &server.read().await.clients;
        let mut other_players: Vec<Arc<RwLock<Client>>> = Vec::new();
        let num_players = all_players.len();
        println!("Num Players: {:?}", num_players);
        for i in 0..num_players {
            let player = all_players[i].clone();
            println!("Testing player if it is the client {:?}, {:?}", i, player.read().await.name);
            if player.read().await.id != client_id {
                println!("Testing player if it is connected {:?}, {:?}", i, player.read().await.name);
                if player.read().await.connected {
                    println!("Adding player {:?}, {:?}", i, player.read().await.name);
                    other_players.push(player.clone());
                }
            }
        }
        for c in other_players {
            println!("To Player: {:?}", c.read().await.name);
            let result = c.read().await.send(&packet_header, &copied_packet).await;
            if !result {
                println!("{:?} disconnected due to client", c.read().await.name);
                c.write().await.connected = false;
            }
        }
        println!("Sent packets");
    }

    pub async fn broadcast_raw(server: Arc<RwLock<Server>>, data: &[u8], size: usize, client: Arc<RwLock<Client>>)
    {
        println!("broadcast_raw");

        println!("From Player: {:?}", client.read().await.name);

        let client_id = client.read().await.id;
        let all_players = &server.read().await.clients;
        let mut other_players: Vec<Arc<RwLock<Client>>> = Vec::new();
        let num_players = all_players.len();
        println!("Num Players: {:?}", num_players);
        for i in 0..num_players {
            let player = all_players[i].clone();
            println!("Testing player if it is the client {:?}, {:?}", i, player.read().await.name);
            if player.read().await.id != client_id {
                println!("Testing player if it is connected {:?}, {:?}", i, player.read().await.name);
                if player.read().await.connected {
                    println!("Adding player {:?}, {:?}", i, player.read().await.name);
                    other_players.push(player.clone());
                }
            }
        }
        for c in other_players {
            println!("To Player: {:?}", c.read().await.name);
            let result = c.read().await.send_raw_data(data, size).await;
            if !result {
                println!("{:?} disconnected due to client", c.read().await.name);
                c.write().await.connected = false;
            }
        }
        println!("Sent packets");
    }

    fn mario_size(is_2d: bool) -> f32 {
        println!("mario_size");
        if is_2d {
            return 180.0;
        }
        return 160.0;
    }
}