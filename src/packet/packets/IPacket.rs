use nalgebra::*;

pub struct IPacket<T> {
    pub packet_key: String,
    pub packet_size: usize,
    pub packet: T,
}

pub trait IPacketTrait {
    fn new() -> Self;
    fn serialize(&self) -> [u8; 1024];
    fn deserialize(&mut self, data: &[u8]);
    fn get_name(&self) -> &str;
    fn get_size(&self) -> &usize;
    fn byte_to_bool(&self, data: u8) -> bool {
        return data != 0x0;
    }
    fn bool_to_byte(&self, data: bool) -> u8 {
        if data { 0x1 } else { 0x0 }
    }
    fn string_to_bytes<const SIZE: usize>(&self, data: String) -> [u8; SIZE] {
        let mut returning_data: [u8; SIZE] = [0; SIZE];
        let string_bytes = data.into_bytes();
        returning_data[..string_bytes.len()].copy_from_slice(string_bytes.as_slice());
        return returning_data;
    }
    fn bytes_to_string(&self, data: &[u8]) -> String {
        let end_pos_result = data.iter().position(|n| n == &0u8);
        let string_result = match end_pos_result {
            Some(end_pos) => {
                String::from_utf8(data[..end_pos].to_vec())
            },
            None => {
                String::from_utf8(data.to_vec())
            }
        };

        return match string_result {
            Ok(string) => string,
            Err(err) => {
                let up_to = err.utf8_error().valid_up_to();
                self.bytes_to_string(&data[..up_to])
            },
        }
    }
    fn bytes_to_vec3(&self, data: &[u8]) -> Vector3<f32> {
        let mut pos_x: [u8; 4] = [0; 4];
        pos_x.copy_from_slice(&data[..4]);
        let mut pos_y: [u8; 4] = [0; 4];
        pos_y.copy_from_slice(&data[4..8]);
        let mut pos_z: [u8; 4] = [0; 4];
        pos_z.copy_from_slice(&data[8..12]);
        return Vector3::new(
            f32::from_le_bytes(pos_x),
            f32::from_le_bytes(pos_y),
            f32::from_le_bytes(pos_z)
        );
    }
    fn vec3_to_bytes(&self, data: Vector3<f32>) -> [u8; 12] {
        let mut returning_data: [u8; 12] = [0x0; 12];

        let position_x_bytes = data.x.to_le_bytes();
        returning_data[..4].copy_from_slice(&position_x_bytes);
        let position_y_bytes = data.y.to_le_bytes();
        returning_data[4..8].copy_from_slice(&position_y_bytes);
        let position_z_bytes = data.z.to_le_bytes();
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
            f32::from_le_bytes(rot_w),
            f32::from_le_bytes(rot_i),
            f32::from_le_bytes(rot_j),
            f32::from_le_bytes(rot_k)
        );
    }
    fn quad_to_bytes(&self, data: Quaternion<f32>) -> [u8; 16] {
        let mut returning_data: [u8; 16] = [0x0; 16];

        let rotation_w_bytes = data.w.to_le_bytes();
        returning_data[..4].copy_from_slice(&rotation_w_bytes);
        let rotation_i_bytes = data.i.to_le_bytes();
        returning_data[4..8].copy_from_slice(&rotation_i_bytes);
        let rotation_j_bytes = data.j.to_le_bytes();
        returning_data[8..12].copy_from_slice(&rotation_j_bytes);
        let rotation_k_bytes = data.k.to_le_bytes();
        returning_data[12..16].copy_from_slice(&rotation_k_bytes);

        return returning_data;
    }

    fn copy(&self) -> Self
    where Self: Sized
    {
        let mut copied_packet = Self::new();
        let data = self.serialize();
        copied_packet.deserialize(&data.as_slice());
        return copied_packet;
    }
}