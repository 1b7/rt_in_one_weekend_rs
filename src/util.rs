// This is known as rtweekend.h in the text.

use crate::vec3::Colour;

pub fn degrees_to_radians(deg: f32) -> f32 {
    deg * std::f32::consts::PI / 180.0
}

/// Returns a random double in the range from `min` up to, but **not including**, `max`.
pub fn random_double(min: f32, max: f32) -> f32 {
    min + (max - min) * fastrand::Rng::new().f32()
}

pub fn throbber(x: usize) -> &'static str {
    match x % 6 {
        0 => "  .",
        1 => " ..",
        2 => "...",
        3 => ".. ",
        4 => ".  ",
        5 => "   ",
        _ => unreachable!()
    }
}

pub fn col_as_rgb(col: &Colour, samples_per_pixel: usize) -> [u8; 3] {
    let scale = 1.0 / samples_per_pixel as f32;
    [col.x(), col.y(), col.z()]
        .map(|x| ((x * scale).sqrt().clamp(0.0, 0.999) * 256.0) as u8)
}