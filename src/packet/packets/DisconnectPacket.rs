use crate::packet::packets::IPacket::{
    IPacketTrait,
    IPacket
};

pub struct DisconnectPacket { }

const SIZE: usize = 0;
impl IPacketTrait<[u8; SIZE]> for IPacket<DisconnectPacket> {
    fn new() -> Self {
        IPacket {
            packet_key: "DisconnectPacket".to_string(),
            packet_size: SIZE,
            packet: DisconnectPacket {}
        }
    }
    fn get_name(&self) -> &str {
        self.packet_key.as_str()
    }
    fn get_size(&self) -> &usize {
        &self.packet_size
    }
    fn serialize(&self) -> [u8; SIZE] {
        return [0x0; SIZE];
    }
    fn deserialize(&mut self, data: &[u8]) {}
}
