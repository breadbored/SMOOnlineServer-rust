use nalgebra::*;

pub struct IPacket<T> {
    pub packet_size: usize,
    pub packet: T,
}

pub trait IPacketTrait<T = [u8; 128]> {
    fn new() -> Self;
    fn serialize(&self) -> T {
        let data: T;
        return data;
    }
    fn deserialize(&mut self, data: &[u8]) {}
    fn byte_to_bool(&self, data: u8) -> bool {
        return data != 0x0;
    }
    fn bool_to_byte(&self, data: bool) -> u8 {
        if data { 0x1 } else { 0x0 }
    }
    fn string_to_bytes<const SIZE: usize>(&self, data: String) -> [u8; SIZE] {
        let mut returning_data: [u8; SIZE] = [0; SIZE];
        returning_data.copy_from_slice(data.as_bytes());
        return returning_data;
    }
    fn bytes_to_string(&self, data: &[u8]) -> String {
        String::from_utf8(data.to_vec()).unwrap()
    }
    fn bytes_to_vec3(&self, data: &[u8]) -> Vector3<f32> {
        let mut pos_x: [u8; 4] = [0; 4];
        pos_x.copy_from_slice(&data[..4]);
        let mut pos_y: [u8; 4] = [0; 4];
        pos_y.copy_from_slice(&data[4..8]);
        let mut pos_z: [u8; 4] = [0; 4];
        pos_z.copy_from_slice(&data[8..12]);
        return Vector3::new(
            f32::from_ne_bytes(pos_x),
            f32::from_ne_bytes(pos_y),
            f32::from_ne_bytes(pos_z)
        );
    }
    fn vec3_to_bytes(&self, data: Vector3<f32>) -> [u8; 12] {
        let mut returning_data: [u8; 12] = [0x0; 12];

        let position_x_bytes = data.x.to_ne_bytes();
        returning_data[..4].copy_from_slice(&position_x_bytes);
        let position_y_bytes = data.y.to_ne_bytes();
        returning_data[4..8].copy_from_slice(&position_y_bytes);
        let position_z_bytes = data.z.to_ne_bytes();
        returning_data[8..12].copy_from_slice(&position_z_bytes);

        return returning_data;
    }
    fn bytes_to_quad(&self, data: &[u8]) -> Quaternion<f32> {
        let mut rot_w: [u8; 4] = [0; 4];
        rot_w.copy_from_slice(&data[..4]);
        let mut rot_i: [u8; 4] = [0; 4];
        rot_i.copy_from_slice(&data[4..8]);
        let mut rot_j: [u8; 4] = [0; 4];
        rot_j.copy_from_slice(&data[8..12]);
        let mut rot_k: [u8; 4] = [0; 4];
        rot_k.copy_from_slice(&data[12..16]);
        return Quaternion::new(
            f32::from_ne_bytes(rot_w),
            f32::from_ne_bytes(rot_i),
            f32::from_ne_bytes(rot_j),
            f32::from_ne_bytes(rot_k)
        );
    }
    fn quad_to_bytes(&self, data: Quaternion<f32>) -> [u8; 16] {
        let mut returning_data: [u8; 16] = [0x0; 16];

        let rotation_w_bytes = data.w.to_ne_bytes();
        returning_data[..4].copy_from_slice(&rotation_w_bytes);
        let rotation_i_bytes = data.i.to_ne_bytes();
        returning_data[4..8].copy_from_slice(&rotation_i_bytes);
        let rotation_j_bytes = data.j.to_ne_bytes();
        returning_data[8..12].copy_from_slice(&rotation_j_bytes);
        let rotation_k_bytes = data.k.to_ne_bytes();
        returning_data[12..16].copy_from_slice(&rotation_k_bytes);

        return returning_data;
    }
}