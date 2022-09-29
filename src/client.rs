
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
    pub async fn send<T: IPacketTrait>(&mut self, packet_header: &IPacket<PacketHeader>, packet: &T)
    {
        let packet_header_size: usize = packet_header.packet_size as usize;
        let packet_size: usize = packet_header.packet.packet_size as usize;


        let mut raw_data: [u8; 1024] = [0; 1024];
        raw_data[..packet_header_size].copy_from_slice(&packet_header.serialize()[..packet_header_size]);
        raw_data[packet_header_size..(packet_header_size + packet_size)].copy_from_slice(
            &packet.serialize()[..packet_size]
        );

        println!("It's sending");
        println!("{:?}", &raw_data[..(packet_header_size + packet_size)]);
        self.socket.lock().await
            .write_all(&raw_data[..(packet_header_size + packet_size)])
            .await
            .expect("failed to write data to socket");
    }

    pub async fn send_raw_data(&mut self, data: &[u8], size: usize) {
        
        self.socket.lock().await
            .write_all(&data[..size])
            .await
            .expect("failed to write data to socket");
    }


    fn string_to_bytes<const SIZE: usize>(&self, data: String) -> [u8; SIZE] {
        let mut returning_data: [u8; SIZE] = [0; SIZE];
        let string_bytes = data.as_bytes();
        returning_data[..string_bytes.len()].copy_from_slice(string_bytes);
        return returning_data;
    }
    fn bytes_to_string(&self, data: &[u8]) -> String {
        let end_pos = data.iter().position(|n| n == &0u8).unwrap();
        String::from_utf8(data[..end_pos].to_vec()).unwrap()
    }

    pub fn copy(&self) -> Self {
        let cloned_socket = self.socket.clone();
        let mut new_client = Self::new(cloned_socket);
        new_client.connected = self.connected;
        new_client.id = self.id;
        new_client.name = self.name.clone();

        let mut costume_packet = None;
        match &self.current_costume {
            Some(x) => {
                costume_packet = Some(x.copy());
            },
            _ => {}
        }
        new_client.current_costume = costume_packet;

        let mut last_packet = None;
        match &self.metadata.last_game_packet {
            Some(x) => {
                last_packet = Some(x.copy());
            },
            _ => {}
        }
        new_client.metadata = Metadata {
            shine_sync: self.metadata.shine_sync.clone(),
            loaded_save: self.metadata.loaded_save,
            scenario: self.metadata.scenario,
            is_2d: self.metadata.is_2d,
            speedrun: self.metadata.speedrun,
            last_game_packet: last_packet,
            seeking: self.metadata.seeking,
            time: Time {
                minutes: self.metadata.time.minutes,
                seconds: self.metadata.time.seconds,
                when: self.metadata.time.when,
            },
        };
        return new_client;
    }
}

impl Clone for Client {
    fn clone(&self) -> Self {
        self.copy()
    }
}