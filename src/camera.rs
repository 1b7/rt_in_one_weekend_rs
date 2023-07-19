use crate::{Vec3, Point3, ray::Ray, util::degrees_to_radians, vec3::{unit_vector, cross, random_in_unit_disk}};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3, v: Vec3, w: Vec3,
    lens_radius: f32
}

impl Camera {
    pub fn new(look_from: Point3, look_at: Point3, vup: Vec3, vertical_fov: f32, aspect_ratio: f32, aperture: f32, focus_dist: f32) -> Self {
        let theta = degrees_to_radians(vertical_fov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(&(look_from - look_at));
        let u = unit_vector(&cross(&vup, &w));
        let v = cross(&w, &u);
        
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;

        Self {
            origin: look_from,
            horizontal,
            vertical,
            lower_left_corner: look_from - horizontal/2.0 - vertical / 2.0 - focus_dist * w,
            u, v, w,
            lens_radius: aperture / 2.0
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.origin + offset, 
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset
        )
    }
}