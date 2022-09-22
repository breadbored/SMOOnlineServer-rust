use crate::packet::packets::IPacket::{
    IPacketTrait,
    IPacket
};
use nalgebra::*;

const SIZE: usize = 0x38;
const FLOAT32_SIZE: usize = 4;
const ANIMATION_WEIGHT_SIZE: usize = 24 / FLOAT32_SIZE;
const ACT_SIZE: usize = 0x20;
const SUB_ACT_SIZE: usize = 0x10;

pub struct PlayerPacket {
    pub position: Vector3<f32>,
    pub rotation: Quaternion<f32>,
    pub animation_blend_weights: [f32; ANIMATION_WEIGHT_SIZE],
    pub act: u16,
    pub sub_act: u16,
}

pub fn convert_u32_to_u8(source: &[f32; ANIMATION_WEIGHT_SIZE]) -> [u8; ANIMATION_WEIGHT_SIZE * FLOAT32_SIZE] {
    let mut dest: [u8; ANIMATION_WEIGHT_SIZE * FLOAT32_SIZE] = [0; ANIMATION_WEIGHT_SIZE * FLOAT32_SIZE];
    for (dest_c, source_e) in dest.chunks_exact_mut(FLOAT32_SIZE).zip(source.iter()) {
        dest_c.copy_from_slice(&source_e.to_ne_bytes())
    }
    dest
}

fn as_u32_ne(array: &[u8; ANIMATION_WEIGHT_SIZE * FLOAT32_SIZE]) -> [f32; ANIMATION_WEIGHT_SIZE] {
    let mut returning_data: [f32; ANIMATION_WEIGHT_SIZE] = [0.0; ANIMATION_WEIGHT_SIZE];
    for i in 0..(ANIMATION_WEIGHT_SIZE * FLOAT32_SIZE) {
        let mut temp: [u8; FLOAT32_SIZE] = [0; FLOAT32_SIZE];
        temp.copy_from_slice(&array[(i * FLOAT32_SIZE)..(i * FLOAT32_SIZE + FLOAT32_SIZE)]);
        returning_data[i] = f32::from_ne_bytes(temp);
    }
    return returning_data;
}

impl IPacketTrait<[u8; SIZE]> for IPacket<PlayerPacket> {
    fn new() -> Self {
        IPacket {
            packet_key: "PlayerPacket".to_string(),
            packet_size: SIZE,
            packet: PlayerPacket {
                position: Vector3::new(0.0,0.0,0.0),
                rotation: Quaternion::new(0.0,0.0,0.0,0.0),
                animation_blend_weights: [0.0; ANIMATION_WEIGHT_SIZE],
                act: 0,
                sub_act: 0,
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

        returning_data[..12].copy_from_slice(&self.vec3_to_bytes(self.packet.position));
        returning_data[12..28].copy_from_slice(&self.quad_to_bytes(self.packet.rotation));
        let offset = 28 + ANIMATION_WEIGHT_SIZE * FLOAT32_SIZE;
        returning_data[28..offset].copy_from_slice(&convert_u32_to_u8(&self.packet.animation_blend_weights));
        returning_data[offset..(offset + 2)].copy_from_slice(&u16::to_ne_bytes(self.packet.act));
        returning_data[(offset + 2)..(offset + 4)].copy_from_slice(&u16::to_ne_bytes(self.packet.sub_act));

        return returning_data;
    }
    fn deserialize(&mut self, data: &[u8]) {
        self.packet.position = self.bytes_to_vec3(&data[..12]);
        self.packet.rotation = self.bytes_to_quad(&data[12..28]);

        let offset = 28 + ANIMATION_WEIGHT_SIZE * FLOAT32_SIZE;

        let mut animation_blend_weights: [u8; ANIMATION_WEIGHT_SIZE * FLOAT32_SIZE] = [0; ANIMATION_WEIGHT_SIZE * FLOAT32_SIZE];
        animation_blend_weights.copy_from_slice(&data[28..offset]);
        self.packet.animation_blend_weights = as_u32_ne(&animation_blend_weights);

        let mut temp: [u8; 2] = [0; 2];
        temp.copy_from_slice(&data[offset..(offset + 2)]);
        self.packet.act = u16::from_ne_bytes(temp);
        temp.copy_from_slice(&data[(offset + 2)..(offset + 4)]);
        self.packet.sub_act = u16::from_ne_bytes(temp);
    }
}
