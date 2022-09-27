pub mod ipacket {
    use crate::packet::packets::IPacket::ipacket::
    {
        IPacket,
        IPacketTrait,
    };

    pub struct ShinePacket {
        pub shine_id: u32,
    }

    const SIZE: usize = 4;
    impl IPacketTrait for IPacket<ShinePacket> {
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
        fn serialize(&self) -> [u8; 1024] {
            let mut returning_data: [u8; 1024] = [0x0; 1024];
            returning_data[..SIZE].copy_from_slice(&u32::to_le_bytes(self.packet.shine_id));
            return returning_data;
        }
        fn deserialize(&mut self, data: &[u8]) {
            let mut arr: [u8; SIZE] = [0; SIZE];
            arr.copy_from_slice(&data[..SIZE]);
            self.packet.shine_id = u32::from_le_bytes(arr);
        }
    }
}