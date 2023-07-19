use std::rc::Rc;

use crate::material::Material;

use super::ray::*;
use super::{Vec3, dot, Point3};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub t: f32,
    pub front_face: bool
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, material: Rc<dyn Material>, t: f32, front_face: bool) -> Self {
        Self { p, normal, material, t, front_face }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}