use std::fs::File;
use std::io::prelude::*;


pub fn slurp(path: &str) -> String {
    let mut file = File::open(path).expect(&format!("Failed to open assets file: {}", path));
    let mut data = String::new();
    file.read_to_string(&mut data).expect(&format!("Failed to read assets file: {}", path));
    data
}


pub fn slurp_bytes(path: &str) -> Vec<u8> {
    let mut file = File::open(path).expect(&format!("Failed to open assets file: {}", path));
    let mut data = vec![];
    file.read_to_end(&mut data).expect(&format!("Failed to read assets file: {}", path));
    data
}