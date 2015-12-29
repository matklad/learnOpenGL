#![allow(unused_variables)]

use std::path::Path;
use std::collections::HashMap;

use glium::backend::glutin_backend::GlutinFacade;
use glium::uniforms::Uniforms;
use glium::vertex::BufferCreationError as VertexBufferCreationError;
use glium::index::BufferCreationError as IndexBufferCreationError;
use glium::texture::Texture2d;
use glium::{Surface, DrawError, Program};
use itertools::Itertools;

use assets::load_texture;
use tobj::{self, Material};

use Api;

mod mesh;

use self::mesh::Mesh;


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
        let textures = load_textures(facade, &base, &materials);
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

fn load_textures(facade: &GlutinFacade,
                 base_path: &Path,
                 materials: &[tobj::Material])
                 -> Textures {
    materials.iter()
             .filter_map(|material| {
                 match material.diffuse_texture.as_ref() {
                     "" => None,
                     tex => {
                         let image = load_texture(base_path.join(tex));
                         let texture = Texture2d::new(facade, image).unwrap();
                         Some((tex.to_owned(), texture))
                     }
                 }
             })
             .collect::<HashMap<_, _>>()
}
