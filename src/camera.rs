use glium::glutin::{Event, ElementState, VirtualKeyCode};
use cgmath::{Point3, Point, Matrix4, Vector, Vector2, EuclideanVector, vec2, vec3, Deg, Angle,
             Quaternion, zero, Rotation3, Rad};

use math::{look_at, Vec3, Mat4};

type Vec2 = Vector2<f32>;

pub struct Camera {
    eye: Point3<f32>,
    up: Vec3,

    speed: f32,
    sensitivity: f32,

    previous_mouse_position: Option<Vec2>,
    pitch: Deg<f32>,
    yaw: Deg<f32>,
}

impl Camera {
    pub fn new(eye: Vec3, center: Vec3, up: Vec3) -> Camera {
        let q = Quaternion::from_sv(1.0, (center - eye).normalize());
        Camera {
            eye: Point3::from_vec(eye),
            up: up,
            speed: 8.0,
            sensitivity: 20000.0,
            pitch: Deg::from(q.to_euler().0),
            yaw: Deg::from(q.to_euler().1),
            previous_mouse_position: None,
        }
    }

    pub fn view(&self) -> Mat4 {
        Mat4(Matrix4::look_at(self.eye, self.eye + self.front(), self.up))
    }

    pub fn rotation(&self) -> Mat4 {
        Mat4(Matrix4::from(Quaternion::from_euler(self.pitch.into(), self.yaw.into(), Rad::zero())))
    }

    pub fn position(&self) -> Vec3 {
        self.eye.to_vec()
    }

    pub fn position_unif(&self) -> [f32; 3] {
        self.eye.into()
    }

    fn right(&self) -> Vec3 {
        self.front().cross(self.up).normalize()
    }

    fn front(&self) -> Vec3 {
        vec3(self.yaw.cos() * self.pitch.cos(),
             self.pitch.sin(),
             self.yaw.sin() * self.pitch.cos())
    }


    pub fn process_event(&mut self, event: Event, delta_t: f32) {
        match event {
            Event::KeyboardInput(ElementState::Pressed, _, Some(code)) => {
                self.process_key(code, delta_t)
            }
            Event::MouseMoved(p) => self.process_mouse(pixel_to_relative(p), delta_t),
            _ => {}
        }
    }

    fn process_key(&mut self, code: VirtualKeyCode, delta_t: f32) {
        let direction = match code {
            VirtualKeyCode::W => self.front(),
            VirtualKeyCode::S => -self.front(),
            VirtualKeyCode::D => self.right(),
            VirtualKeyCode::A => -self.right(),
            _ => Vec3::zero(),
        };
        self.eye = self.eye + direction * self.speed * delta_t;
    }

    fn process_mouse(&mut self, new_position: Vec2, delta_t: f32) {
        if let Some(prev) = self.previous_mouse_position {
            let delta = (new_position - prev) * self.sensitivity * delta_t;
            self.yaw = self.yaw + Deg::new(delta.x);
            self.pitch = self.pitch + Deg::new(-delta.y);
        }
        self.previous_mouse_position = Some(new_position);
    }
}

fn pixel_to_relative((x, y): (i32, i32)) -> Vec2 {
    let width = 800.0;
    let height = 600.0;
    vec2(x as f32 / width, y as f32 / height)
}
