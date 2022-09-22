use crate::packet::packets::IPacket::{
    IPacketTrait,
    IPacket
};

pub struct CapturePacket {
    pub module_name: String,
}

const SIZE: usize = 0x20;
impl IPacketTrait for IPacket<CapturePacket> {
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
    fn serialize(&self) -> [u8; 1024] {
        let mut returning_data: [u8; 1024] = [0x0; 1024];
        returning_data[..SIZE].copy_from_slice(&self.string_to_bytes::<SIZE>(self.packet.module_name.to_string()));
        return returning_data;
    }
    fn deserialize(&mut self, data: &[u8]) {
        self.packet.module_name = self.bytes_to_string(data);
    }
}
