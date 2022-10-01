
use std::{
    hash::{Hash, Hasher},
    collections::hash_map::DefaultHasher,
    sync::{
        Arc, 
        // Mutex
    }
};
use async_trait::async_trait;
use tokio::{
    sync::Mutex,
    net::TcpStream,
    io::AsyncWriteExt
};
use uuid::Uuid;
use chrono::{
    DateTime,
    Utc
};
use crate::{
    packet::{packets::{
        GamePacket::{GamePacket},
        CostumePacket::CostumePacket,
        IPacket::{IPacketTrait, IPacket}
    }, PacketHeader::PacketHeader}
};

pub struct Time {
    pub minutes: u16,
    pub seconds: u8,
    pub when: DateTime<Utc>,
}

pub struct Metadata {
    pub shine_sync: Vec<usize>,
    pub loaded_save: bool,
    pub scenario: u8,
    pub is_2d: bool,
    pub speedrun: bool,
    pub last_game_packet: Option<IPacket<GamePacket>>,
    pub seeking: bool,
    pub time: Time,
}

pub struct Client {
    pub metadata: Metadata,
    pub connected: bool,
    pub current_costume: Option<IPacket<CostumePacket>>,
    pub name: String,
    pub id: Uuid,
    pub socket: Arc<Mutex<TcpStream>>,
}

#[async_trait]
pub trait ClientTraits {
    fn new(socket: Arc<Mutex<TcpStream>>) -> Client;
    fn get_hash_code(&self) -> u64;
}

impl PartialEq for Client {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
    fn ne(&self, other: &Self) -> bool {
        self.id != other.id
    }
}

impl ClientTraits for Client {
    fn new(socket: Arc<Mutex<TcpStream>>) -> Client {
        Client {
            metadata: Metadata {
                shine_sync: vec![],
                loaded_save: false, 
                scenario: 200, 
                is_2d: false, 
                speedrun: false, 
                last_game_packet: None, 
                seeking: false, 
                time: Time {
                    minutes: 0,
                    seconds: 0,
                    when: Utc::now(),
                }
            },
            socket: socket,
            connected: false,
            current_costume: None,
            name: "".to_string(),
            id: Uuid::new_v4(),
            // server: &server,
        }
    }

    fn get_hash_code(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.id.hash(&mut hasher);
        return hasher.finish();
    }
}

impl Client {
    pub async fn send<T: IPacketTrait>(&self, packet_header: &IPacket<PacketHeader>, packet: &T) -> bool
    {
        let packet_header_size: usize = packet_header.packet_size as usize;
        let packet_size: usize = packet_header.packet.packet_size as usize;


        let mut raw_data: [u8; 1024] = [0; 1024];
        raw_data[..packet_header_size].copy_from_slice(&packet_header.serialize()[..packet_header_size]);
        raw_data[packet_header_size..(packet_header_size + packet_size)].copy_from_slice(
            &packet.serialize()[..packet_size]
        );

        // println!("It's sending");
        // println!("{:?}", &raw_data[..(packet_header_size + packet_size)]);
        let result = self.socket.lock().await
            .write_all(&raw_data[..(packet_header_size + packet_size)])
            .await;
        
        return match result {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub async fn send_raw_data(&mut self, data: &[u8], size: usize) -> bool {
        
        let result = self.socket.lock().await
            .write_all(&data[..size])
            .await;
        
        return match result {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}