use uuid::Uuid;

use crate::packet::{
    PacketType::PacketType,
    packets::IPacket::{
        IPacketTrait,
        IPacket
    }
};

pub struct PacketHeader {
    pub id: Uuid,
    pub packet_type: PacketType,
    pub packet_size: u16
}

pub const SIZE: usize = 20;
impl IPacketTrait for IPacket<PacketHeader> {
    fn new() -> Self {
        IPacket {
            packet_key: "PacketHeader".to_string(),
            packet_size: SIZE,
            packet: PacketHeader {
                id: Uuid::new_v4(),
                packet_type: PacketType::Unknown,
                packet_size: 0
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

        returning_data[..16].copy_from_slice(&self.packet.id.as_bytes().as_slice());

        let packet_type: u16 = self.packet.packet_type as u16;
        returning_data[16..18].copy_from_slice(&packet_type.to_be_bytes());

        returning_data[18..SIZE].copy_from_slice(&self.packet.packet_size.to_be_bytes());

        return returning_data;
    }
    fn deserialize(&mut self, data: &[u8]) {
        let mut id: [u8; 16] = [0; 16];
        id.copy_from_slice(&data[..16]);
        self.packet.id = Uuid::from_bytes(id);

        let mut packet_type: [u8; 2] = [0; 2];
        packet_type.copy_from_slice(&data[16..18]);
        self.packet.packet_type = u16::from_le_bytes(packet_type).try_into().unwrap();
        
        let mut packet_size: [u8; 2] = [0; 2];
        packet_size.copy_from_slice(&data[18..SIZE]);
        self.packet.packet_size = u16::from_le_bytes(packet_size);
    }
}
