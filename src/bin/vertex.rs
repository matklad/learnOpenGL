#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
//    normal: [f32; 3],
}

implement_vertex!(Vertex, position);

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32,
//    nx: f32, ny: f32, nz: f32
    ) -> Vertex {
        Vertex {
            position: [x, y, z],
//            normal: [nx, ny, nz],
        }
    }

    pub fn many(verts: Vec<f32>) -> Vec<Vertex> {
        let m = 3;
        if verts.len() % m != 0 {
            panic!("Number of coordinates should be divisible by {}, but it was {}",
                   m,
                   verts.len())
        }
        verts.chunks(m)
             .map(|p| Vertex::new(p[0], p[1], p[2],
//    p[3], p[4], p[5]
    ))
             .collect()
    }
}
