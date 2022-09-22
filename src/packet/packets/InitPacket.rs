use crate::packet::packets::IPacket::{
    IPacketTrait,
    IPacket
};

pub struct InitPacket {
    pub max_players: u16,
}

const SIZE: usize = 0x2;
impl IPacketTrait<[u8; SIZE]> for IPacket<InitPacket> {
    fn new() -> Self {
        IPacket {
            packet_key: "InitPacket".to_string(),
            packet_size: SIZE,
            packet: InitPacket {
                max_players: 0,
            }
        }
    }
    fn get_name(&self) -> &str {
        self.packet_key.as_str()
    }
    fn get_size(&self) -> &usize {
        &self.packet_size
    }
    fn serialize(&self) -> [u8; SIZE] {
        return self.packet.max_players.to_ne_bytes();
    }
    fn deserialize(&mut self, data: &[u8]) {
        let mut arr: [u8; 2] = [0; 2];
        arr.copy_from_slice(data);
        self.packet.max_players = u16::from_ne_bytes(arr);
    }
}
