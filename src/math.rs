use cgmath::{Basis3, Rotation3, Matrix4, Vector3};

use cgmath::{self, Rad, EuclideanVector};
use glium::uniforms;

pub use cgmath::{vec3, deg};

pub struct Mat4(Matrix4<f32>);

pub const X: Vector3<f32> = Vector3 {
    x: 1.0,
    y: 0.0,
    z: 0.0,
};

pub const Y: Vector3<f32> = Vector3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

pub const Z: Vector3<f32> = Vector3 {
    x: 0.0,
    y: 0.0,
    z: 1.0,
};

pub fn perspective<R: Into<Rad<f32>>>(fov: R, aspect_ratio: f32, near: f32, far: f32) -> Mat4 {
    Mat4(cgmath::perspective(fov.into(), aspect_ratio, near, far))
}


pub fn id() -> Mat4 {
    Mat4(Matrix4::from_scale(1.0))
}

impl Mat4 {
    pub fn scale(&self, scale: f32) -> Mat4 {
        Mat4(self.0 * Matrix4::from_scale(scale))
    }

    pub fn rotate<R: Into<Rad<f32>>>(&self, axis: Vector3<f32>, angle: R) -> Mat4 {
        let rotation = Basis3::from_axis_angle(axis.normalize(), angle.into());
        Mat4(self.0 * Matrix4::from(rotation.as_ref().clone()))
    }

    pub fn translate(&self, direction: Vector3<f32>) -> Mat4 {
        Mat4(self.0 * Matrix4::from_translation(direction))
    }
}

impl uniforms::AsUniformValue for Mat4 {
    fn as_uniform_value(&self) -> uniforms::UniformValue {
        uniforms::UniformValue::Mat4(self.0.into())
    }
}