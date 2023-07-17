// This is known as rtweekend.h in the text.

pub const PI: f32 = 3.1415926535897932385;

pub fn degrees_to_radians(deg: f32) -> f32 {
    deg * PI / 180.0
}

/// Returns a random double in the range from `min` up to, but **not including**, `max`.
pub fn random_double(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rand::Rng::gen_range(&mut rng, min..max)
}