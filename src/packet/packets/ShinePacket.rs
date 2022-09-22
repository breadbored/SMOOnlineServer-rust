use crate::packet::packets::IPacket::{
    IPacketTrait,
    IPacket
};

pub struct ShinePacket {
    pub shine_id: u32,
}

const SIZE: usize = 4;
impl IPacketTrait<[u8; SIZE]> for IPacket<ShinePacket> {
    fn new() -> Self {
        IPacket {
            packet_key: "ShinePacket".to_string(),
            packet_size: SIZE,
            packet: ShinePacket {
                shine_id: 0
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
        return u32::to_ne_bytes(self.packet.shine_id);
    }
    fn deserialize(&mut self, data: &[u8]) {
        let mut arr: [u8; SIZE] = [0; SIZE];
        arr.copy_from_slice(data);
        self.packet.shine_id = u32::from_ne_bytes(arr);
    }
}
