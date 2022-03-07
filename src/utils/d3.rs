use crate::color::Color;
use crate::math::*;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::result::Result;

/// A parser of object files containing any number of triangles (*.obj)
/// Currently only triangles are supported.
pub struct Object3D {
    pub mesh: Mesh3D,
}

impl Object3D {
    /// Create a 3D Object from a given obj file with triangles.
    /// Only triangles are currently supported on the file.
    /// `filename`  file containing the triangle mesh for the object
    pub fn from_file(filename: &str) -> Result<Object3D, Box<dyn Error>> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let mut tris = Vec::<Triangle3D>::new();
        let mut vertices = Vec::<FVec3D>::new();

        for line in reader.lines().flatten() {
            // reading vertices from obj file
            // formated as: v 0.00 1.00 2.00
            if line.starts_with('v') {
                let tokens: Vec<String> = line
                    .split_ascii_whitespace()
                    .map(|s| s.to_string())
                    .collect();
                let x: f32 = tokens[1].parse()?;
                let y: f32 = tokens[2].parse()?;
                let z: f32 = tokens[3].parse()?;
                vertices.push(FVec3D::new(x, y, z));
            }
            // reading triangles from obj file
            // formated as: f 1 2 3 where each number is an index
            // to the vertices read earlier (starting at 1)
            else if line.starts_with('f') {
                let tokens: Vec<String> = line
                    .split_ascii_whitespace()
                    .map(|s| s.to_string())
                    .collect();
                let x: usize = tokens[1].parse()?;
                let y: usize = tokens[2].parse()?;
                let z: usize = tokens[3].parse()?;
                let triangle = Triangle3D {
                    vertices: [vertices[x - 1], vertices[y - 1], vertices[z - 1]],
                    color: Color::rgb(170, 248, 11),
                };
                tris.push(triangle);
            }
        }
        Ok(Object3D {
            mesh: Mesh3D { tris, vertices },
        })
    }
}

/// A triangle implementation in 3 dimensions
#[derive(Default, Clone, Copy)]
pub struct Triangle3D {
    pub vertices: [FVec3D; 3],
    pub color: Color,
}

/// A mesh of triangles
#[derive(Default, Clone)]
pub struct Mesh3D {
    pub tris: Vec<Triangle3D>,
    pub vertices: Vec<FVec3D>,
}
