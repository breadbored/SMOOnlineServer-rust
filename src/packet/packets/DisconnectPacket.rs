pub mod ipacket {
    use crate::packet::packets::IPacket::ipacket::
    {
        IPacket,
        IPacketTrait,
    };

    pub struct DisconnectPacket { }

    const SIZE: usize = 0;
    impl IPacketTrait for IPacket<DisconnectPacket> {
        fn new() -> Self {
            IPacket {
                packet_key: "DisconnectPacket".to_string(),
                packet_size: SIZE,
                packet: DisconnectPacket {}
            }
        }
        fn get_name(&self) -> &str {
            self.packet_key.as_str()
        }
        fn get_size(&self) -> &usize {
            &self.packet_size
        }
        fn serialize(&self) -> [u8; 1024] {
            return [0x0; 1024];
        }
        fn deserialize(&mut self, data: &[u8]) {}
    }
}