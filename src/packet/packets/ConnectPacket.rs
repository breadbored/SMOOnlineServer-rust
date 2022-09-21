use crate::packet::packets::IPacket::{
    IPacketTrait,
    IPacket
};

#[repr(u16)]
pub enum ConnectionTypes {
    FirstConnection = 0x0,
    Reconnecting = 0x1,
}

pub struct ConnectPacket {
    pub connection_type: ConnectionTypes,
    pub max_players: u16,
    pub client_name: String,
}

const SIZE: usize = 0x26;
const NAME_SIZE: usize = 0x20;
impl IPacketTrait<[u8; SIZE]> for IPacket<ConnectPacket> {
    fn new() -> Self {
        IPacket {
            packet_size: SIZE,
            packet: ConnectPacket {
                connection_type: ConnectionTypes::FirstConnection,
                max_players: 0,
                client_name: "?????".to_string(),
            }
        }
    }
    fn serialize(&self) -> [u8; SIZE] {
        let mut returning_data: [u8; SIZE] = [0x0; SIZE];
        
        match self.packet.connection_type {
            ConnectionTypes::FirstConnection => returning_data[..4].copy_from_slice(&(0 as u32).to_ne_bytes()),
            ConnectionTypes::Reconnecting => returning_data[..4].copy_from_slice(&(1 as u32).to_ne_bytes()),
        }

        returning_data[4..6].copy_from_slice(&self.packet.max_players.to_ne_bytes());

        returning_data[6..].copy_from_slice(&self.string_to_bytes::<NAME_SIZE>(self.packet.client_name.to_string()));

        return returning_data;
    }
    fn deserialize(&mut self, data: &[u8]) {
        let mut connection_type_bytes: [u8; 4] = [0; 4];
        connection_type_bytes.copy_from_slice(&data[..4]);
        if u32::from_ne_bytes(connection_type_bytes) == 0 {
            self.packet.connection_type = ConnectionTypes::FirstConnection;
        } else if u32::from_ne_bytes(connection_type_bytes) == 1 {
            self.packet.connection_type = ConnectionTypes::Reconnecting;
        }

        let mut max_players_bytes: [u8; 2] = [0; 2];
        max_players_bytes.copy_from_slice(&data[4..6]);
        self.packet.max_players = u16::from_ne_bytes(max_players_bytes);

        self.packet.client_name = self.bytes_to_string(&data[6..]);
    }
}
