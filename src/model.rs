#![allow(unused_variables)]

use std::path::Path;
use std::collections::HashMap;

use glium::backend::glutin_backend::GlutinFacade;
use glium::index::{PrimitiveType, NoIndices};
use glium::uniforms::{Uniforms, UniformValue, AsUniformValue};
use glium::vertex::BufferCreationError as VertexBufferCreationError;
use glium::index::BufferCreationError as IndexBufferCreationError;
use glium::texture::Texture2d;
use glium::{Surface, DrawError, VertexBuffer, Program};

use assets::load_texture;
use tobj::{self, Material};

use Api;

quick_error! {
    #[derive(Debug)]
    pub enum ModelLoadingError {
        VertexBufferCreationError(err: VertexBufferCreationError) {
            from()
            cause(err)
        }
        IndexBufferCreationError(err: IndexBufferCreationError) {
            from()
        }
        LoadError(err: tobj::LoadError) {
            from()
        }
    }
}

type Textures = HashMap<String, Texture2d>;


#[derive(Debug)]
pub struct Model {
    meshes: Vec<Mesh>,
    materials: Vec<Material>,
    textures: Textures,
}

impl Model {
    pub fn load<P: AsRef<Path>>(facade: &GlutinFacade,
                                path: P)
                                -> Result<Model, ModelLoadingError> {
        let models_path = Path::new("./assets/models/");
        let model_path = models_path.join(path);
        let base = model_path.parent().expect("Invalid model path");

        let (models, materials) = try!(tobj::load_obj(&model_path));
        let meshes = try!(models.into_iter()
                                .map(|m| Mesh::from_obj(facade, m))
                                .collect::<Result<Vec<_>, _>>());

        let textures = materials.iter()
                                .filter_map(|material| {
                                    match material.diffuse_texture.as_ref() {
                                        "" => None,
                                        tex => {
                                            let image = load_texture(base.join(tex));
                                            let texture = Texture2d::new(facade, image).unwrap();
                                            Some((tex.to_owned(), texture))
                                        }
                                    }
                                })
                                .collect::<HashMap<_, _>>();
        Ok(Model {
            meshes: meshes,
            materials: materials,
            textures: textures,
        })
    }

    pub fn draw<S: Surface, U: Uniforms>(&self,
                                         api: &mut Api<S>,
                                         program: &Program,
                                         uniforms: &U)
                                         -> Result<(), DrawError> {
        for m in &self.meshes {
            let material = m.material_id.map(|i| &self.materials[i]);
            let ref tex = self.textures[&material.unwrap().diffuse_texture];
            try!(m.draw(api, program, uniforms, material, tex))
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Mesh {
    vertex_buffer: VertexBuffer<Vertex>,
    material_id: Option<usize>,
}

impl Mesh {
    fn from_obj(facade: &GlutinFacade, model: tobj::Model) -> Result<Mesh, ModelLoadingError> {
        let mut vertices = vec![];
        let ref mesh = model.mesh;
        for i in mesh.indices.iter().map(|&i| i as usize) {
            let (a, b, c) = (i * 3, i * 3 + 1, i * 3 + 2);
            vertices.push(Vertex {
                position: [mesh.positions[a], mesh.positions[b], mesh.positions[c]],
                normal: [mesh.normals[a], mesh.normals[b], mesh.normals[c]],
                texture: [mesh.texcoords[i * 2], mesh.texcoords[i * 2 + 1]],
            });

        }

        Ok(Mesh {
            vertex_buffer: try!(VertexBuffer::new(facade, &vertices)),
            material_id: model.mesh.material_id,
        })
    }
    pub fn draw<S: Surface, U: Uniforms>(&self,
                                         api: &mut Api<S>,
                                         program: &Program,
                                         uniforms: &U,
                                         material: Option<&Material>,
                                         texture: &Texture2d)
                                         -> Result<(), DrawError> {


        api.surface.draw(&self.vertex_buffer,
                         &NoIndices(PrimitiveType::TrianglesList),
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
    material: Option<&'a Material>,
    texture: &'a Texture2d,
    u: &'a U,
}

impl<'a, U: Uniforms> Uniforms for MyUniform<'a, U> {
    fn visit_values<'c, F: FnMut(&str, UniformValue<'c>)>(&'c self, mut f: F) {
        if let Some(material) = self.material {
            f("color_ambient", material.ambient.as_uniform_value());
            f("color_diffuse", material.diffuse.as_uniform_value());
            f("texture_diffuse", self.texture.as_uniform_value());
            f("color_specular", material.specular.as_uniform_value());
            f("shininess", material.shininess.as_uniform_value());
        }
        self.u.visit_values(f);
    }
}
