use super::vec3::*;

pub fn write_colour(col: &Colour, samples_per_pixel: usize) {
    let scale = 1.0 / samples_per_pixel as f32;
    let rgb = [col.x(), col.y(), col.z()]
        .map(|n| ((n * scale).clamp(0.0, 0.999) * 256.0) as u8);

    println!("{} {} {}", rgb[0], rgb[1], rgb[2]);
}