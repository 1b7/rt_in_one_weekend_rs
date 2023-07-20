use std::sync::Arc;

use crate::{vec3::*, hittable::{Hittable, HitRecord}, material::Material};

#[derive(Clone)]
pub struct Triangle {
    points: [Point3; 3],
    material: Arc<dyn Material>
}

impl Triangle {
    pub fn new(a: Point3, b: Point3, c: Point3, material: Arc<dyn Material>) -> Self {
        Self { points: [a, b, c], material }
    }

    fn normal(&self) -> Vec3 {
        let ab = self.points[1] - self.points[0];
        let ac = self.points[2] - self.points[0];
        unit_vector(&cross(&ab, &ac))
    }
}

impl Hittable for Triangle {
    fn hit(&self, r: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<crate::hittable::HitRecord> {
        let norm = self.normal();
        let d = -dot(&self.normal(), &self.points[0]);

        // Reject rays which are parallel to the triangle:
        let denom = dot(&norm, &r.direction());
        if denom.abs() < 1e-08 { return None }

        let t = -(dot(&norm, &r.origin()) + d) / denom;
        if t < t_min || t_max < t {
            return None;
        }

        let p = r.origin() + t * r.direction();

        // Hit-testing using 'inside-outside' test:
        let ab = self.points[1] - self.points[0];
        let bc = self.points[2] - self.points[1];
        let ca = self.points[0] - self.points[2];

        let pa = p - self.points[0];
        let pb = p - self.points[1];
        let pc = p - self.points[2];

        if dot(&norm, &cross(&ab, &pa)) > 0.0 
            && dot(&norm, &cross(&bc, &pb)) > 0.0 
            && dot(&norm, &cross(&ca, &pc)) > 0.0 
        {
            Some(HitRecord::new(p, self.normal(), self.material.clone(), t, true))
        } else { None }   
    }
}