use crate::packet::packets::IPacket::{
    IPacketTrait,
    IPacket
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
impl IPacketTrait<[u8; SIZE]> for IPacket<CapPacket> {
    fn new() -> Self {
        IPacket {
            packet: CapPacket {
                position: Vector3::new(0.0,0.0,0.0),
                rotation: Quaternion::new(0.0,0.0,0.0,0.0),
                cap_out: false,
                cap_animation: "".to_string(),
            }
        }
    }
    fn serialize(&self) -> [u8; SIZE] {
        let mut returning_data: [u8; SIZE] = [0x0; SIZE];

        returning_data[..12].copy_from_slice(&self.vec3_to_bytes(self.packet.position));

        returning_data[12..28].copy_from_slice(&self.quad_to_bytes(self.packet.rotation));

        returning_data[28] = self.bool_to_byte(self.packet.cap_out);

        returning_data[32..].copy_from_slice(&self.string_to_bytes::<NAME_SIZE>(self.packet.cap_animation.to_string()));

        return returning_data;
    }
    fn deserialize(mut self, data: &mut [u8]) {
        self.packet.position = self.bytes_to_vec3(&data[..12]);
        self.packet.rotation = self.bytes_to_quad(&data[12..28]);
        
        self.packet.cap_out = self.byte_to_bool(data[28]);

        self.packet.cap_animation = self.bytes_to_string(&data[32..(32 + NAME_SIZE)]);
    }
}
