use std::error;
use std::fmt;
use std::io::prelude::*;
use std::io;
use std::result;

#[derive(Debug)]
pub struct Obj {
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub indices: Vec<u16>,
}

#[derive(Debug)]
pub enum ObjError {
    Io(io::Error),
    SyntaxError(String),
    NotSupported,
}

pub type Result<T> = result::Result<T, ObjError>;

pub fn parse(data: &str) -> Result<Obj> {
    let mut result = Obj {
        vertices: Vec::new(),
        normals: Vec::new(),
        indices: Vec::new(),
    };

    for line in data.lines() {
        if line.starts_with("v ") {
            result.vertices.push(try!(parse_vec(line)))
        } else if line.starts_with("vn ") {
            result.normals.push(try!(parse_vec(line)))
        } else if line.starts_with("f ") {
            result.indices.extend(try!(parse_face(line)).iter())
        }
    }

    Ok(result)
}

fn parse_vec(line: &str) -> Result<[f32; 3]> {
    let coords = try!(line.split_whitespace()
                          .skip(1)
                          .map(|s| {
                              s.parse::<f32>().map_err(|_| ObjError::SyntaxError(line.to_owned()))
                          })
                          .collect::<Result<Vec<_>>>());

    if coords.len() != 3 {
        return Err(ObjError::SyntaxError(line.to_owned()));
    }

    Ok([coords[0], coords[1], coords[2]])
}

fn parse_face(line: &str) -> Result<Vec<u16>> {
    let verts = try!(line.split_whitespace()
                         .skip(1)
                         .map(parse_index)
                         .collect::<Result<Vec<_>>>());

    if verts.len() != 3 {
        return Err(ObjError::SyntaxError(line.to_owned()));
    }

    Ok(verts)
}

fn parse_index(s: &str) -> Result<u16> {
    let inds = try!(s.split("//")
                     .map(|i| {
                         i.parse::<u16>()
                          .map(|i| i - 1)
                          .map_err(|_| ObjError::SyntaxError(s.to_owned()))
                     })
                     .collect::<Result<Vec<_>>>());
    if inds.len() != 2 {
        return Err(ObjError::SyntaxError(s.to_owned()));
    }
    if inds[0] != inds[1] {
        return Err(ObjError::NotSupported);
    }
    Ok(inds[0])
}


impl fmt::Display for ObjError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ObjError::Io(ref err) => write!(f, "IO error: {}", err),
            ObjError::SyntaxError(ref s) => write!(f, "Syntax Error in {}", s),
            ObjError::NotSupported => write!(f, "Feature not supported"),
        }
    }
}

impl error::Error for ObjError {
    fn description(&self) -> &str {
        match *self {
            ObjError::Io(ref err) => err.description(),
            ObjError::SyntaxError(_) => "Syntax Error",
            ObjError::NotSupported => "Feature not supported",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ObjError::Io(ref err) => Some(err),
            ObjError::SyntaxError(_) | ObjError::NotSupported => None,
        }
    }
}

impl From<io::Error> for ObjError {
    fn from(err: io::Error) -> ObjError {
        ObjError::Io(err)
    }
}
