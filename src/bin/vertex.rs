#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
    texture: [f32; 2],
}

implement_vertex!(Vertex, position, color, texture);

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32, r: f32, g: f32, b: f32, tx: f32, ty: f32) -> Vertex {
        Vertex {
            position: [x, y, z],
            color: [r, g, b],
            texture: [tx, ty],
        }
    }

    pub fn many(verts: Vec<f32>) -> Vec<Vertex> {
        let m = 8;
        if verts.len() % m != 0 {
            panic!("Number of coordinates should be divisible by {}, but it was {}",
                   m,
                   verts.len())
        }
        verts.chunks(m)
             .map(|p| Vertex::new(p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7]))
             .collect()
    }
}
