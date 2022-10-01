use crate::packet::packets::IPacket::{
    IPacketTrait,
    IPacket
};

pub struct GamePacket {
    pub is_2d: bool,
    pub scenario_num: u8,
    pub stage: String,
}

const SIZE: usize = 0x42;
const STAGE_SIZE: usize = 0x40;

impl IPacketTrait for IPacket<GamePacket> {
    fn new() -> Self {
        IPacket {
            packet_key: "GamePacket".to_string(),
            packet_size: SIZE,
            packet: GamePacket {
                is_2d: false,
                scenario_num: 0,
                stage: "".to_string(),
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
        returning_data[0] = self.bool_to_byte(self.packet.is_2d);
        returning_data[1] = self.packet.scenario_num;
        returning_data[2..(2 + STAGE_SIZE)].copy_from_slice(&self.string_to_bytes::<STAGE_SIZE>(self.packet.stage.to_string()));
        return returning_data;
    }
    fn deserialize(&mut self, data: &[u8]) {
        self.packet.is_2d = self.byte_to_bool(data[0]);
        self.packet.scenario_num = data[1];
        self.packet.stage = self.bytes_to_string(&data[2..SIZE]);
    }
}
