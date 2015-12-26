use std::io::Cursor;

use glium::texture::RawImage2d;

use image;

pub fn load_texture_jpeg(bytes: &[u8]) -> RawImage2d<'static, u8> {
    let im = image::load(Cursor::new(bytes), image::JPEG)
                 .expect("Failed to load a jpeg texture")
                 .to_rgba();
    let dim = im.dimensions();
    RawImage2d::from_raw_rgba_reversed(im.into_raw(), dim)
}
