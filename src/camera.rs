use crate::{Vec3, Point3, ray::Ray, util::degrees_to_radians, vec3::{unit_vector, cross}};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new(look_from: Point3, look_at: Point3, vup: Vec3, vertical_fov: f32, aspect_ratio: f32) -> Self {
        let theta = degrees_to_radians(vertical_fov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(&(look_from - look_at));
        let u = unit_vector(&cross(&vup, &w));
        let v = cross(&w, &u);
        
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;

        Self {
            origin: look_from,
            horizontal,
            vertical,
            lower_left_corner: look_from - horizontal/2.0 - vertical / 2.0 - w
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin, 
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin
        )
    }
}