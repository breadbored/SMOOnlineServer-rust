use crate::packet::packets::IPacket::{
    IPacketTrait,
    IPacket
};

pub struct CostumePacket {
    pub body_name: String,
    pub cap_name: String,
}

const COSTUME_SIZE: usize = 0x20;
const SIZE: usize = COSTUME_SIZE * 2;
impl IPacketTrait<[u8; SIZE]> for IPacket<CostumePacket> {
    fn new() -> Self {
        IPacket {
            packet_key: "CostumePacket".to_string(),
            packet_size: SIZE,
            packet: CostumePacket {
                body_name: "".to_string(),
                cap_name: "".to_string(),
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
        let mut returning_data: [u8; SIZE] = [0x0; SIZE];

        returning_data[..COSTUME_SIZE].copy_from_slice(&self.string_to_bytes::<COSTUME_SIZE>(self.packet.body_name.to_string()));
        returning_data[COSTUME_SIZE..].copy_from_slice(&self.string_to_bytes::<COSTUME_SIZE>(self.packet.cap_name.to_string()));

        return returning_data;
    }
    fn deserialize(&mut self, data: &[u8]) {
        self.packet.body_name = self.bytes_to_string(&data[..COSTUME_SIZE]);
        self.packet.cap_name = self.bytes_to_string(&data[COSTUME_SIZE..]);
    }
}
