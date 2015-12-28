use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;

use glium::{Program, ProgramCreationError};
use glium::backend::Facade;
use glium::backend::glutin_backend::GlutinFacade;
use glium::texture::cubemap::Cubemap;
use glium::texture::{UncompressedFloatFormat, MipmapsOption, Dimensions, RawImage2d};
use gl;

use image;

use {obj, Obj};

type RawImage = RawImage2d<'static, u8>;


pub fn load_program(facade: &GlutinFacade,
                    vertex_shader_path: &str,
                    fragment_shader_path: &str)
                    -> Result<Program, ProgramCreationError> {

    let vertex_shader = slurp(&format!("./assets/shaders/{}", vertex_shader_path));
    let fragment_shader = slurp(&format!("./assets/shaders/{}", fragment_shader_path));
    (Program::from_source(facade, &vertex_shader, &fragment_shader, None))
}

pub fn load_obj(obj_file: &str) -> Obj {
    let data = slurp(&format!("./assets/models/{}", obj_file));
    obj::parse(&data).unwrap()
}

pub fn load_cubemap(facade: &GlutinFacade, texture_src: &str) -> Cubemap {
    info!("Loading cubemap {} ...", texture_src);
    let parts = ["right", "left", "bottom", "top", "back", "front"];
    let mut size = 0;
    let faces = parts.iter()
                     .map(|part| {
                         let path = &format!("./assets/textures/{}/{}.jpg", texture_src, part);
                         let im = image::load(Cursor::new(slurp_bytes(path)), image::JPEG)
                                      .expect("Failed to load a texture")
                                      .to_rgba();
                         let dim = im.dimensions();
                         if size == 0 {
                             size = dim.0;
                         }
                         if size != dim.0 || size != dim.1 {
                             panic!("Bad cubemap texture size: {:?}", dim);
                         }

                         RawImage2d::from_raw_rgba_reversed(im.into_raw(), dim)
                     })
                     .collect::<Vec<_>>();

    let result = unsafe {
        let mut id = 0;
        facade.get_context().exec_in_context(|| {
            let window = facade.get_window().expect("cant load cubemap in headless context");
            gl::load_with(|s| window.get_proc_address(s) as *const _);

            id = cubemap_id(faces, size)
        });
        debug!("Cubemap id {}", id);
        Cubemap::from_id(facade,
                         UncompressedFloatFormat::U8U8U8U8,
                         id,
                         true,
                         MipmapsOption::NoMipmap,
                         Dimensions::Cubemap { dimension: size })
    };
    info!("    ...Done!");
    result
}

unsafe fn cubemap_id(faces: Vec<RawImage>, size: u32) -> u32 {
    let mut result: u32 = 0;
    gl::GenTextures(1, &mut result);
    gl::BindTexture(gl::TEXTURE_CUBE_MAP, result);
    for (i, face) in faces.iter().enumerate() {
        let bind_point = gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32;
        let size = size as i32;
        gl::TexImage2D(bind_point,
                       0,
                       gl::RGBA as i32,
                       size,
                       size,
                       0,
                       gl::RGBA,
                       gl::UNSIGNED_BYTE,
                       face.data.as_ptr() as *const _);
    }

    gl::TexParameteri(gl::TEXTURE_CUBE_MAP,
                      gl::TEXTURE_MAG_FILTER,
                      gl::LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_CUBE_MAP,
                      gl::TEXTURE_MIN_FILTER,
                      gl::LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_CUBE_MAP,
                      gl::TEXTURE_WRAP_S,
                      gl::CLAMP_TO_EDGE as i32);
    gl::TexParameteri(gl::TEXTURE_CUBE_MAP,
                      gl::TEXTURE_WRAP_T,
                      gl::CLAMP_TO_EDGE as i32);
    gl::TexParameteri(gl::TEXTURE_CUBE_MAP,
                      gl::TEXTURE_WRAP_R,
                      gl::CLAMP_TO_EDGE as i32);
    gl::BindTexture(gl::TEXTURE_CUBE_MAP, 0);

    result
}

pub fn slurp(path: &str) -> String {
    let mut file = File::open(path).expect(&format!("Failed to open assets file: {}", path));
    let mut data = String::new();
    file.read_to_string(&mut data).expect(&format!("Failed to read assets file: {}", path));
    data
}


fn slurp_bytes(path: &str) -> Vec<u8> {
    let mut file = File::open(path).expect(&format!("Failed to open assets file: {}", path));
    let mut data = vec![];
    file.read_to_end(&mut data).expect(&format!("Failed to read assets file: {}", path));
    data
}
