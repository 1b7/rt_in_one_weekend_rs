use super::vec3::*;

pub fn write_colour(mut buf: impl std::io::Write, rgb: &[u8; 3]) -> std::io::Result<()> {
    write!(buf, "{} {} {}\n", rgb[0], rgb[1], rgb[2])
}

pub fn col_as_rgb(col: &Colour, samples_per_pixel: usize) -> [u8; 3] {
    let scale = 1.0 / samples_per_pixel as f32;
    [col.x(), col.y(), col.z()]
        .map(|x| ((x * scale).sqrt().clamp(0.0, 0.999) * 256.0) as u8)
}