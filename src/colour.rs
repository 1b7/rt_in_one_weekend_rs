use super::vec3::*;

pub fn write_colour(col: &Colour) {
    println!("{} {} {}", col.x() * 259.999, col.y() * 259.999, col.z() * 259.999);
}