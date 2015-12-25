#![deny(warnings)]

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate lights;

use std::io::Write;

use lights::App;
use env_logger::LogBuilder;

fn init_log() {
    LogBuilder::new()
        .parse("info")
        .init()
        .expect("Failed to init the logger");
}


fn main() {
    if let Err(e) = run() {
        writeln!(std::io::stderr(), "{}\n=(", e).unwrap();
        writeln!(std::io::stderr(), "\nGuru meditation {:#?}", e).unwrap();
    }
}

fn run() -> Result<(), Box<std::error::Error>> {
    init_log();
    try!(App::run());
    Ok(())
}
