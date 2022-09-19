use crate::packet::packets::IPacket::{
    IPacketTrait,
    IPacket
};

pub struct UnhandledPacket { }

const SIZE: usize = 0;
impl IPacketTrait<[u8; SIZE]> for IPacket<UnhandledPacket> {
    fn new() -> Self {
        IPacket {
            packet: UnhandledPacket {}
        }
    }
    fn serialize(&self) -> [u8; SIZE] {
        return [0x0; SIZE];
    }
    fn deserialize(mut self, data: &mut [u8]) {}
}
