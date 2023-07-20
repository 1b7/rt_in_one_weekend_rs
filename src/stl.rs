use std::{fs, io};

use crate::vec3::Point3;

/// Returns the triangles stored in some STL file as a vector of 3-arrays of `Point3`s.
/// 
/// This is a very simple and naive reader, which can only read ASCII encoded files.
pub fn import(fp: &str) -> io::Result<Vec<[Point3; 3]>> {
    Ok(get_triangles(&fs::read_to_string(fp)?))
}

fn get_triangles(text: &str) -> Vec<[Point3; 3]> {
    let lines: Vec<_> = text.lines().skip(1).collect();
    lines.chunks_exact(7).map(|chunk| {
        let points: Vec<_> = chunk[2..=4].iter().map(to_vertex).collect();
        [points[0], points[1], points[2]]
    }).collect()
}

fn to_vertex(line: &&str) -> Point3 {
    assert!(line.starts_with("vertex"));
    let ns: Vec<f32> = line.split_whitespace().skip(1)
        .map(|n| n.parse::<f32>().unwrap())
        .collect();
    
    Point3::new(ns[0], ns[1], ns[2])
}