pub mod ipacket {
    use crate::packet::packets::IPacket::ipacket::
    {
        IPacket,
        IPacketTrait,
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
    impl IPacketTrait for IPacket<ChangeStagePacket> {
        fn new() -> Self {
            IPacket {
                packet_key: "ChangeStagePacket".to_string(),
                packet_size: SIZE,
                packet: ChangeStagePacket {
                    stage: "".to_string(),
                    id: "".to_string(),
                    scenario: 0,
                    sub_scenario_type: 0,
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
            
            returning_data[..STAGE_SIZE].copy_from_slice(&self.string_to_bytes::<STAGE_SIZE>(self.packet.stage.to_string()));
            returning_data[STAGE_SIZE..(ID_SIZE + STAGE_SIZE)].copy_from_slice(&self.string_to_bytes::<ID_SIZE>(self.packet.id.to_string()));
            returning_data[(ID_SIZE + STAGE_SIZE)..(ID_SIZE + STAGE_SIZE + 1)].copy_from_slice(&self.packet.scenario.to_le_bytes());
            returning_data[(ID_SIZE + STAGE_SIZE + 1)..(ID_SIZE + STAGE_SIZE + 2)].copy_from_slice(&self.packet.sub_scenario_type.to_le_bytes());

            return returning_data;
        }
        fn deserialize(&mut self, data: &[u8]) {
            self.packet.stage = self.bytes_to_string(&data[..STAGE_SIZE]);
            self.packet.id = self.bytes_to_string(&data[STAGE_SIZE..(ID_SIZE + STAGE_SIZE)]);
            self.packet.scenario = i8::from_le_bytes([data[(ID_SIZE + STAGE_SIZE)]; 1]);
            self.packet.sub_scenario_type = data[(ID_SIZE + STAGE_SIZE + 1)];
        }
    }
}