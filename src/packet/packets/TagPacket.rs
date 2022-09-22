use crate::packet::packets::IPacket::{
    IPacketTrait,
    IPacket
};

#[repr(u8)]
pub enum TagUpdate {
    Time = 0x1,
    State = 0x2,
}

pub struct TagPacket {
    pub update_type: TagUpdate,
    pub is_it: bool,
    pub seconds: u8,
    pub minutes: u16,
}

const SIZE: usize = 6;
impl IPacketTrait<[u8; SIZE]> for IPacket<TagPacket> {
    fn new() -> Self {
        IPacket {
            packet_key: "TagPacket".to_string(),
            packet_size: SIZE,
            packet: TagPacket {
                update_type: TagUpdate::Time,
                is_it: false,
                seconds: 0,
                minutes: 0,
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
        
        match self.packet.update_type {
            TagUpdate::Time => returning_data[0] = 1,
            TagUpdate::State => returning_data[0] = 0,
        }
        
        returning_data[1] = self.bool_to_byte(self.packet.is_it);
        returning_data[2] = self.packet.seconds;
        returning_data[4..6].copy_from_slice(&u16::to_ne_bytes(self.packet.minutes));

        return returning_data;
    }
    fn deserialize(&mut self, data: &[u8]) {
        if data[0] == 1 {
            self.packet.update_type = TagUpdate::Time;
        } else if data[0] == 2 {
            self.packet.update_type = TagUpdate::State;
        }

        self.packet.is_it = self.byte_to_bool(data[1]);
        self.packet.seconds = data[2];
        
        let mut minutes: [u8; 2] = [0; 2];
        minutes.copy_from_slice(&data[4..6]);
        self.packet.minutes = u16::from_ne_bytes(minutes);
    }
}
