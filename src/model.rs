#![allow(unused_variables)]

use glium::backend::glutin_backend::GlutinFacade;
use glium::index::{PrimitiveType, IndexBuffer};
use glium::uniforms::Uniforms;
use glium::vertex::BufferCreationError as VertexBufferCreationError;
use glium::index::BufferCreationError as IndexBufferCreationError;
use glium::{Surface, DrawError, VertexBuffer, Program};

use wavefront_obj::obj::{self, Object, Shape};
use wavefront_obj::ParseError;

use {assets, Api};

quick_error! {
    #[derive(Debug)]
    pub enum ModelLoadingError {
        AssetError(err: assets::AssetLoadingError) {
            from()
            cause(err)
        }
        VertexBufferCreationError(err: VertexBufferCreationError) {
            from()
            cause(err)
        }
        IndexBufferCreationError(err: IndexBufferCreationError) {
            from()
        }
        ParseError(err: ParseError) {
            from()
        }
    }
}

#[derive(Debug)]
pub struct Model {
    meshes: Vec<Mesh>,
}

impl Model {
    pub fn load(facade: &GlutinFacade, path: &str) -> Result<Model, ModelLoadingError> {
        let bytes = try!(assets::slurp(&format!("./assets/models/{}", path)));
        let objset = try!(obj::parse(bytes));

        let meshes = try!(objset.objects
                                .into_iter()
                                .map(|o| Mesh::from_obj(facade, o))
                                .collect::<Result<Vec<_>, _>>());

        Ok(Model { meshes: meshes })
    }

    pub fn draw<S: Surface, U: Uniforms>(&self,
                                         api: &mut Api<S>,
                                         program: &Program,
                                         uniforms: &U)
                                         -> Result<(), DrawError> {
        for m in &self.meshes {
            try!(m.draw(api, program, uniforms))
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Mesh {
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u16>,
}

impl Mesh {
    fn from_obj(facade: &GlutinFacade, obj: Object) -> Result<Mesh, ModelLoadingError> {
        let vertices = obj.vertices
                          .into_iter()
                          .map(Vertex::from)
                          .collect::<Vec<_>>();
        let indices = obj.geometry
                         .into_iter()
                         .flat_map(|g| {
                             g.shapes.into_iter().flat_map(|s| {
                                 match s {
                                     Shape::Triangle(a, b, c) => vec![a, b, c].into_iter(),
                                     _ => panic!("unsupported shape"),
                                 }
                             })
                         })
                         .map(|(v, _, _)| v as u16)
                         .collect::<Vec<_>>();

        Ok(Mesh {
            vertex_buffer: try!(VertexBuffer::new(facade, &vertices)),
            index_buffer: try!(IndexBuffer::new(facade, PrimitiveType::TrianglesList, &indices)),
        })
    }
}

impl Mesh {
    pub fn draw<S: Surface, U: Uniforms>(&self,
                                         api: &mut Api<S>,
                                         program: &Program,
                                         uniforms: &U)
                                         -> Result<(), DrawError> {

        api.surface.draw(&self.vertex_buffer,
                         &self.index_buffer,
                         program,
                         uniforms,
                         &api.default_params)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: [f32; 3],
}

impl From<obj::Vertex> for Vertex {
    fn from(v: obj::Vertex) -> Vertex {
        Vertex { position: [v.x as f32, v.y as f32, v.z as f32] }
    }
}

implement_vertex!(Vertex, position);
