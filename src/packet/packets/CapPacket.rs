pub mod ipacket {
    use crate::packet::packets::IPacket::ipacket::
    {
        IPacket,
        IPacketTrait,
    };

    use nalgebra::*;

    pub struct CapPacket {
        pub position: Vector3<f32>,
        pub rotation: Quaternion<f32>,
        pub cap_out: bool,
        pub cap_animation: String,
    }

    const SIZE: usize = 0x50;
    const NAME_SIZE: usize = 0x30;
    impl IPacketTrait for IPacket<CapPacket> {
        fn new() -> Self {
            IPacket {
                packet_key: "CapPacket".to_string(),
                packet_size: SIZE,
                packet: CapPacket {
                    position: Vector3::new(0.0,0.0,0.0),
                    rotation: Quaternion::new(0.0,0.0,0.0,0.0),
                    cap_out: false,
                    cap_animation: "".to_string(),
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

            returning_data[..12].copy_from_slice(&self.vec3_to_bytes(self.packet.position));

            returning_data[12..28].copy_from_slice(&self.quad_to_bytes(self.packet.rotation));

            returning_data[28] = self.bool_to_byte(self.packet.cap_out);

            returning_data[32..SIZE].copy_from_slice(&self.string_to_bytes::<NAME_SIZE>(self.packet.cap_animation.to_string()));

            return returning_data;
        }
        fn deserialize(&mut self, data: &[u8]) {
            self.packet.position = self.bytes_to_vec3(&data[..12]);
            self.packet.rotation = self.bytes_to_quad(&data[12..28]);
            
            self.packet.cap_out = self.byte_to_bool(data[28]);

            self.packet.cap_animation = self.bytes_to_string(&data[32..(32 + NAME_SIZE)]);
        }
    }
}