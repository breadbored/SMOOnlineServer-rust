use crate::packet::packets::IPacket::{
    IPacketTrait,
    IPacket
};

pub struct ChangeStagePacket {
    pub stage: String,
    pub id: String,
    pub scenario: i8,
    pub sub_scenario_type: u8,
}

const SIZE: usize = 0x44;
const ID_SIZE: usize = 0x10;
const STAGE_SIZE: usize = 0x30;
impl IPacketTrait<[u8; SIZE]> for IPacket<ChangeStagePacket> {
    fn new() -> Self {
        IPacket {
            packet: ChangeStagePacket {
                stage: "".to_string(),
                id: "".to_string(),
                scenario: 0,
                sub_scenario_type: 0,
            }
        }
    }
    fn serialize(&self) -> [u8; SIZE] {
        let mut returning_data: [u8; SIZE] = [0x0; SIZE];
        
        returning_data[..STAGE_SIZE].copy_from_slice(&self.string_to_bytes::<STAGE_SIZE>(self.packet.stage.to_string()));
        returning_data[STAGE_SIZE..(ID_SIZE + STAGE_SIZE)].copy_from_slice(&self.string_to_bytes::<ID_SIZE>(self.packet.id.to_string()));
        returning_data[(ID_SIZE + STAGE_SIZE)..(ID_SIZE + STAGE_SIZE + 1)].copy_from_slice(&self.packet.scenario.to_ne_bytes());
        returning_data[(ID_SIZE + STAGE_SIZE + 1)..(ID_SIZE + STAGE_SIZE + 2)].copy_from_slice(&self.packet.sub_scenario_type.to_ne_bytes());

        return returning_data;
    }
    fn deserialize(mut self, data: &mut [u8]) {
        self.packet.stage = self.bytes_to_string(&data[..STAGE_SIZE]);
        self.packet.id = self.bytes_to_string(&data[STAGE_SIZE..(ID_SIZE + STAGE_SIZE)]);
        self.packet.scenario = i8::from_ne_bytes([data[(ID_SIZE + STAGE_SIZE)]; 1]);
        self.packet.sub_scenario_type = data[(ID_SIZE + STAGE_SIZE + 1)];
    }
}
