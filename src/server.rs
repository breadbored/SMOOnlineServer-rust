use std::{net::TcpStream, cell::RefCell, rc::Weak};
use uuid::Uuid;
use chrono::{
    DateTime,
    Utc
};
use crate::{packet::packets::{
    GamePacket::GamePacket,
    CostumePacket::CostumePacket
}, client::Client};


pub struct Server<'a> {
    pub clients: Vec<Client<'a>>
}
pub trait ServerTraits {

}

impl ServerTraits for Server<'_> {

}