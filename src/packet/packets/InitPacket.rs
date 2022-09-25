use crate::packet::packets::IPacket::{
    IPacketTrait,
    IPacket
};

pub struct InitPacket {
    pub max_players: u16,
}

const SIZE: usize = 0x2;
impl IPacketTrait for IPacket<InitPacket> {
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
    fn serialize(&self) -> [u8; 1024] {
        let mut returning_data: [u8; 1024] = [0x0; 1024];
        returning_data[..SIZE].copy_from_slice(&self.packet.max_players.to_le_bytes());
        return returning_data;
    }
    fn deserialize(&mut self, data: &[u8]) {
        let mut arr: [u8; 2] = [0; 2];
        arr.copy_from_slice(&data[..SIZE]);
        self.packet.max_players = u16::from_le_bytes(arr);
    }
}
