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
            packet_key: "CapturePacket".to_string(),
            packet_size: SIZE,
            packet: CapturePacket {
                module_name: "".to_string(),
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
        return self.string_to_bytes::<SIZE>(self.packet.module_name.to_string());
    }
    fn deserialize(&mut self, data: &[u8]) {
        self.packet.module_name = self.bytes_to_string(data);
    }
}
