use std::f32::consts::PI;
use nalgebra::{Quaternion, Matrix4};

pub trait QuaternionMatrixConvertible {
    fn create_from_rotation_matrix(matrix: Matrix4<f32>) -> Self;
    fn create_from_rotation_matrix_x() -> Self;
    fn create_from_rotation_matrix_y() -> Self;
}
impl QuaternionMatrixConvertible for Quaternion<f32> {
    fn create_from_rotation_matrix(matrix: Matrix4<f32>) -> Self {
        // println!("create_from_rotation_matrix");
        let w: f32 = (1.0 + matrix.m11 + matrix.m22 + matrix.m33).sqrt() / 2.0;
        let w4: f32 = 4.0 * w;
        let i: f32 = (matrix.m32 - matrix.m23) / w4;
        let j: f32 = (matrix.m13 - matrix.m31) / w4;
        let k: f32 = (matrix.m21 - matrix.m12) / w4;
        Quaternion::<f32> {
            w,
            i,
            j,
            k
        }
    }

    fn create_from_rotation_matrix_x() -> Self {
        // println!("create_from_rotation_matrix_x");
        let matrix = Matrix4::<f32>::create_rotation_x(PI);
        return Quaternion::<f32>::create_from_rotation_matrix(matrix);
    }

    fn create_from_rotation_matrix_y() -> Self {
        // println!("create_from_rotation_matrix_y");
        let matrix = Matrix4::<f32>::create_rotation_y(PI);
        return Quaternion::<f32>::create_from_rotation_matrix(matrix);
    }
}
pub trait MatrixConvertible {
    fn create_rotation_x(radians: f32) -> Self;
    fn create_rotation_y(radians: f32) -> Self;
}
impl MatrixConvertible for Matrix4<f32> {
    fn create_rotation_x(radians: f32) -> Self {
        // println!("create_rotation_x");
        Matrix4::<f32>::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, radians.cos(), -radians.sin(), 0.0,
            0.0, radians.sin(), radians.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0
        )
    }
    fn create_rotation_y(radians: f32) -> Self {
        // println!("create_rotation_y");
        Matrix4::<f32>::new(
            radians.cos(), 0.0, radians.sin(), 0.0,
            0.0, 1.0, 0.0, 0.0,
            -radians.sin(), 0.0, radians.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0
        )
    }
}