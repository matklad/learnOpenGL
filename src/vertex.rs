#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Vertex {
        Vertex { position: [x, y, z] }
    }

    pub fn many(verts: Vec<f32>) -> Vec<Vertex> {
        if verts.len() % 3 != 0 {
            panic!("Number of coordinates should be divisible by three, but it was {}",
                   verts.len())
        }
        verts.chunks(3).map(|p| Vertex::new(p[0], p[1], p[2])).collect()
    }
}

implement_vertex!(Vertex, position);
