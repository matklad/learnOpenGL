use std::fs::File;
use std::io::prelude::*;

use glium::{Program, ProgramCreationError};
use glium::backend::Facade;


pub fn load_program<F: Facade>(facade: &F,
                            vertex_shader_path: &str,
                            fragment_shader_path: &str)
                            -> Result<Program, ProgramCreationError> {

    let vertex_shader = slurp(&format!("./src/bin/shaders/{}", vertex_shader_path));
    let fragment_shader = slurp(&format!("./src/bin/shaders/{}", fragment_shader_path));
    (Program::from_source(facade, &vertex_shader, &fragment_shader, None))
}

fn slurp(path: &str) -> String {
    let mut file = File::open(path).expect(&format!("Failed to open assets file: {}", path));
    let mut data = String::new();
    file.read_to_string(&mut data).expect(&format!("Failed to read assets file: {}", path));
    data
}


//fn slurp_bytes(path: &str) -> Vec<u8> {
//    let mut file = File::open(path).expect(&format!("Failed to open assets file: {}", path));
//    let mut data = vec![];
//    file.read_to_end(&mut data).expect(&format!("Failed to read assets file: {}", path));
//    data
//}

