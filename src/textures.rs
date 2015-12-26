use std::io::Cursor;

use glium::texture::RawImage2d;

use image::{self, ImageFormat};


pub fn load_texture_jpeg(bytes: &[u8]) -> RawImage2d<'static, u8> {
    load_texture(bytes, image::JPEG)
}

pub fn load_texture_png(bytes: &[u8]) -> RawImage2d<'static, u8> {
    load_texture(bytes, image::PNG)
}

fn load_texture(bytes: &[u8], format: ImageFormat) -> RawImage2d<'static, u8> {
    let im = image::load(Cursor::new(bytes), format)
                 .expect("Failed to load a texture")
                 .to_rgba();
    let dim = im.dimensions();
    RawImage2d::from_raw_rgba_reversed(im.into_raw(), dim)
}
