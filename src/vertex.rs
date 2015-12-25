#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}

impl Vertex {
    pub fn new(x: f32, y: f32) -> Vertex {
        Vertex { position: [x, y] }
    }
}

implement_vertex!(Vertex, position);
