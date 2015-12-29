use glium::backend::glutin_backend::GlutinFacade;
use glium::index::{PrimitiveType, IndexBuffer};
use glium::uniforms::{Uniforms, UniformValue, AsUniformValue};
use glium::{VertexBuffer, Surface, Texture2d, DrawError, Program};

use Api;

use tobj;
use super::ModelLoadingError;

#[derive(Debug)]
pub struct Mesh {
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u32>,
    pub material_id: Option<usize>,
}

impl Mesh {
    pub fn from_obj(facade: &GlutinFacade, model: tobj::Model) -> Result<Mesh, ModelLoadingError> {

        let ref mesh = model.mesh;
        let mut vertices = vec![];
        let n = mesh.positions.len() / 3;
        let get = |v: &Vec<f32>, i: usize| v.get(i).map(|&i| i).unwrap_or(0.0);
        for i in 0..n {
            vertices.push(Vertex {
                position: [mesh.positions[3 * i],
                           mesh.positions[3 * i + 1],
                           mesh.positions[3 * i + 2]],
                normal: [get(&mesh.normals, 3 * i),
                         get(&mesh.normals, 3 * i + 1),
                         get(&mesh.normals, 3 * i + 2)],
                texture: [get(&mesh.texcoords, 2 * i), get(&mesh.texcoords, 2 * i + 1)],
            })
        }
        Ok(Mesh {
            vertex_buffer: try!(VertexBuffer::new(facade, &vertices)),
            index_buffer: try!(IndexBuffer::new(facade,
                                                PrimitiveType::TrianglesList,
                                                &mesh.indices)),
            material_id: model.mesh.material_id,
        })
    }
    pub fn draw<S: Surface, U: Uniforms>(&self,
                                         api: &mut Api<S>,
                                         program: &Program,
                                         uniforms: &U,
                                         material: Option<&tobj::Material>,
                                         texture: &Texture2d)
                                         -> Result<(), DrawError> {

        api.surface.draw(&self.vertex_buffer,
                         &self.index_buffer,
                         program,
                         &MyUniform {
                             material: material,
                             texture: texture,
                             u: uniforms,
                         },
                         &api.default_params)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    texture: [f32; 2],
}

implement_vertex!(Vertex, position, normal, texture);

struct MyUniform<'a, U: Uniforms + 'a> {
    material: Option<&'a tobj::Material>,
    texture: &'a Texture2d,
    u: &'a U,
}

impl<'a, U: Uniforms> Uniforms for MyUniform<'a, U> {
    fn visit_values<'c, F: FnMut(&str, UniformValue<'c>)>(&'c self, mut f: F) {
        if let Some(material) = self.material {
            f("color_diffuse", material.diffuse.as_uniform_value());
            f("texture_diffuse", self.texture.as_uniform_value());
            f("color_specular", material.specular.as_uniform_value());
            f("shininess", material.shininess.as_uniform_value());
        }
        self.u.visit_values(f);
    }
}
