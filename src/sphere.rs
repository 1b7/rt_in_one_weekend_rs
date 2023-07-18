use crate::hittable::*;
use crate::vec3::*;
use crate::ray::Ray;

pub struct Sphere {
    centre: Point3,
    radius: f32
}


impl Sphere {
    pub fn new(centre: Point3, radius: f32) -> Self {
        Self { centre, radius }
    } 
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.centre;
        let a = r.direction().length_squared();
        let half_b = dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 { return false }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        
        rec.t = root;
        rec.p = r.at(rec.t);

        let outward_normal = (rec.p - self.centre) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        true
    }
}