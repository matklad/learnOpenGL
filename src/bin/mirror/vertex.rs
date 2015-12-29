#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
}

implement_vertex!(Vertex, position);

impl Vertex {
    pub fn many(verts: Vec<f32>) -> Vec<Vertex> {
        let m = 3;
        if verts.len() % m != 0 {
            panic!("Number of coordinates should be divisible by {}, but it was {}",
                   m,
                   verts.len())
        }
        verts.chunks(m)
             .map(|p| Vertex { position: [p[0], p[1], p[2]] })
             .collect()
    }
}

