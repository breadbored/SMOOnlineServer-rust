use crate::packet::packets::IPacket::{
    IPacketTrait,
    IPacket
};

pub struct UnhandledPacket { }

const SIZE: usize = 0;
impl IPacketTrait for IPacket<UnhandledPacket> {
    fn new() -> Self {
        IPacket {
            packet_key: "UnhandledPacket".to_string(),
            packet_size: SIZE,
            packet: UnhandledPacket {}
        }
    }
    fn get_name(&self) -> &str {
        self.packet_key.as_str()
    }
    fn get_size(&self) -> &usize {
        &self.packet_size
    }
    fn serialize(&self) -> [u8; 1024] {
        return [0x0; 1024];
    }
    fn deserialize(&mut self, data: &[u8]) {}
}
