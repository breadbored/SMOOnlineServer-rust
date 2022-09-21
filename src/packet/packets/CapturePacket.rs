use crate::packet::packets::IPacket::{
    IPacketTrait,
    IPacket
};

pub struct CapturePacket {
    pub module_name: String,
}

const SIZE: usize = 0x20;
impl IPacketTrait<[u8; SIZE]> for IPacket<CapturePacket> {
    fn new() -> Self {
        IPacket {
            packet_size: SIZE,
            packet: CapturePacket {
                module_name: "".to_string(),
            }
        }
    }
    fn serialize(&self) -> [u8; SIZE] {
        return self.string_to_bytes::<SIZE>(self.packet.module_name.to_string());
    }
    fn deserialize(&mut self, data: &[u8]) {
        self.packet.module_name = self.bytes_to_string(data);
    }
}
