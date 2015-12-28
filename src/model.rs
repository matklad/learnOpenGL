#![allow(unused_variables)]

use glium::backend::glutin_backend::GlutinFacade;
use glium::index::{PrimitiveType, NoIndices};
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
}

impl Mesh {
    fn from_obj(facade: &GlutinFacade, obj: Object) -> Result<Mesh, ModelLoadingError> {
        let mut vertices = vec![];
        for g in &obj.geometry {
            for s in g.shapes.iter() {
                match s {
                    &Shape::Triangle((v1, _, Some(n1)), (v2, _, Some(n2)), (v3, _, Some(n3))) => {
                        for &(v, n) in [(v1, n1), (v2, n2), (v3, n3)].iter() {
                            let vert = obj.vertices[v];
                            let norm = obj.normals[n];
                            vertices.push(Vertex {
                                position: [vert.x as f32, vert.y as f32, vert.z as f32],
                                normal: [norm.x as f32, norm.y as f32, norm.z as f32],
                            });
                        }
                    }
                    _ => {}
                }
            }
        }
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

        Ok(Mesh { vertex_buffer: try!(VertexBuffer::new(facade, &vertices)) })
    }
}

impl Mesh {
    pub fn draw<S: Surface, U: Uniforms>(&self,
                                         api: &mut Api<S>,
                                         program: &Program,
                                         uniforms: &U)
                                         -> Result<(), DrawError> {

        api.surface.draw(&self.vertex_buffer,
                         &NoIndices(PrimitiveType::TrianglesList),
                         program,
                         uniforms,
                         &api.default_params)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
}

implement_vertex!(Vertex, position, normal);
