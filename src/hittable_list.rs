use std::sync::Arc;

use crate::hittable::*;

#[derive(Clone)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable + Send + Sync>>
}

impl HittableList {
    pub fn new(objects: Vec<Arc<dyn Hittable + Send + Sync>>) -> Self {
        Self { objects }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable + Send + Sync>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_anything = None;

        for object in &self.objects {
            if let Some(hit) =  object.hit(r, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        }

        hit_anything
    }
}