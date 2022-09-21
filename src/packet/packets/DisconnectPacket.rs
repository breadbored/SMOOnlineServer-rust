use crate::packet::packets::IPacket::{
    IPacketTrait,
    IPacket
};

pub struct DisconnectPacket { }

const SIZE: usize = 0;
impl IPacketTrait<[u8; SIZE]> for IPacket<DisconnectPacket> {
    fn new() -> Self {
        IPacket {
            packet_size: SIZE,
            packet: DisconnectPacket {}
        }
    }
    fn serialize(&self) -> [u8; SIZE] {
        return [0x0; SIZE];
    }
    fn deserialize(&mut self, data: &[u8]) {}
}
