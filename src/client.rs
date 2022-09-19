
use std::{net::TcpStream, cell::RefCell, rc::Weak, hash::{Hash, Hasher}, collections::hash_map::DefaultHasher};
use async_trait::async_trait;
use uuid::Uuid;
use chrono::{
    DateTime,
    Utc
};
use crate::{packet::packets::{
    GamePacket::{GamePacket},
    CostumePacket::CostumePacket
}, server::Server};

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
    pub last_game_packet: GamePacket,
    pub seeking: bool,
    pub time: Time,
}

pub struct Client<'a> {
    pub metadata: Metadata,
    pub connected: bool,
    pub current_costume: Option<CostumePacket>,
    pub name: String,
    pub id: Uuid,
    pub socket: TcpStream,
    pub server: Option<Weak<RefCell<Server<'a>>>>,
}

#[async_trait]
pub trait ClientTraits {
    fn new(socket: TcpStream) -> Client<'static>;
    async fn send<IPacket>(packet: IPacket, sender: Client);
    async fn send_raw_data<const SIZE: usize>(data: [u8; SIZE], sender: Client);
    fn get_hash_code(&self) -> u64;
}

impl PartialEq for Client<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
    fn ne(&self, other: &Self) -> bool {
        self.id != other.id
    }
}

impl ClientTraits for Client<'_> {
    fn new(socket: TcpStream) -> Client<'static> {
        Client {
            metadata: Metadata {
                shine_sync: vec![],
                loaded_save: false, 
                scenario: 0, 
                is_2d: false, 
                speedrun: false, 
                last_game_packet: GamePacket {
                    is_2d: false,
                    scenario_num: 0,
                    stage: "".to_string(),
                }, 
                seeking: false, 
                time: Time {
                    minutes: 0,
                    seconds: 0,
                    when: Utc::now(),
                }
            },
            socket,
            connected: false,
            current_costume: None,
            name: "".to_string(),
            id: Uuid::new_v4(),
            server: None,
        }
    }

    fn send< 'async_trait, IPacket>(packet: IPacket, sender: Client) ->  core::pin::Pin<Box<dyn core::future::Future<Output = ()> + core::marker::Send+ 'async_trait> >where IPacket: 'async_trait+  {
        // Need to implement Server before implementing this
        todo!()
    }

    fn send_raw_data< 'async_trait, const SIZE: usize>(data: [u8; SIZE], sender: Client) ->  core::pin::Pin<Box<dyn core::future::Future<Output = ()> + core::marker::Send+ 'async_trait> >  {
        // Need to implement Server before implementing this
        todo!()
    }

    fn get_hash_code(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.id.hash(&mut hasher);
        return hasher.finish();
    }
}