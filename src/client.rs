
use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher, sync::{Arc}};
use async_trait::async_trait;
use tokio::{sync::Mutex, net::TcpStream, io::AsyncWriteExt};
use uuid::Uuid;
use chrono::{
    DateTime,
    Utc
};
use crate::{
    packet::packets::{
        GamePacket::{GamePacket},
        CostumePacket::CostumePacket,
        IPacket::{IPacketTrait, IPacket}
    }
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
    pub async fn send<T: IPacketTrait>(&mut self, packet: T)
    {
        let packet_size: usize = packet.get_size().to_owned();
        println!("{:?}", &packet.serialize()[..packet_size]);
        self.socket.lock().await
            .write_all(&packet.serialize()[..packet_size])
            .await
            .expect("failed to write data to socket");
    }

    pub async fn send_raw_data(&mut self, data: &[u8], size: usize) {
        self.socket.lock().await
            .write_all(&data[..size])
            .await
            .expect("failed to write data to socket");
    }
}