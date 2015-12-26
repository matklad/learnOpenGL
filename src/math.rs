use cgmath::{Basis3, Rotation3, Matrix4, Vector3};

pub use cgmath::{vec3, Rad, Angle};

pub struct Mat4(Matrix4<f32>);

pub fn id() -> Mat4 {
    Mat4(Matrix4::from_scale(1.0))
}

impl Mat4 {
    pub fn scale(&self, scale: f32) -> Mat4 {
        Mat4(self.0 * Matrix4::from_scale(scale))
    }

    pub fn rotate(&self, axis: Vector3<f32>, rad: Rad<f32>) -> Mat4 {
        let rotation = Basis3::from_axis_angle(axis, rad);
        Mat4(self.0 * Matrix4::from(rotation.as_ref().clone()))
    }

    pub fn into_uniform(self) -> [[f32; 4]; 4] {
        self.0.into()
    }
}


