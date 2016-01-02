use glium::{Surface, Program, VertexBuffer, IndexBuffer, DrawParameters};
use glium::uniforms::Uniforms;
use glium::index::PrimitiveType;
use glium::backend::glutin_backend::GlutinFacade;


use lights::{Result, load_program};


#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    texture: [f32; 2],
}

implement_vertex!(Vertex, position, texture);

pub struct Quad {
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u16>,
    program: Program,
}

impl Quad {
    pub fn new(facade: &GlutinFacade) -> Result<Quad> {
        let vertices = [Vertex {
                            position: [-1.0, -1.0],
                            texture: [0.0, 0.0],
                        },
                        Vertex {
                            position: [1.0, -1.0],
                            texture: [1.0, 0.0],
                        },
                        Vertex {
                            position: [1.0, 1.0],
                            texture: [1.0, 1.0],
                        },
                        Vertex {
                            position: [-1.0, 1.0],
                            texture: [0.0, 1.0],
                        }];

        let vertex_buffer = try!(VertexBuffer::new(facade, &vertices));

        let index_buffer = try!(IndexBuffer::new(facade,
                                                 PrimitiveType::TrianglesList,
                                                 &[0u16, 1, 2, 0, 2, 3]));

        Ok(Quad {
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            program: try!(load_program(facade,
                                       "army/light/vertex.glsl",
                                       "army/light/fragment.glsl")),
        })
    }

    pub fn draw<S: Surface, U: Uniforms>(&self,
                                         surface: &mut S,
                                         params: &DrawParameters,
                                         uniforms: &U)
                                         -> Result<()> {

        Ok(try!(surface.draw(&self.vertex_buffer,
                             &self.index_buffer,
                             &self.program,
                             uniforms,
                             params)))
    }
}
