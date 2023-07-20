use crate::{ray::Ray, hittable::HitRecord, vec3::*, util::random_double};


pub trait Material: Sync + Send {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)>;
}

pub struct Lambertian { albedo: Colour }
impl Lambertian {
    pub fn new(albedo: Colour) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)> {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some((Ray::new(rec.p, scatter_direction), self.albedo))
    }
}

pub struct Metal { albedo: Colour, fuzz: f32 }
impl Metal {
    pub fn new(albedo: Colour, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)> {
        let reflected = reflect(unit_vector(&r_in.direction()), rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere());
        let attenuation = self.albedo;

        if dot(&scattered.direction(), &rec.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}

pub struct Dielectric { refractive_index: f32 }
impl Dielectric {
    pub fn new(refractive_index: f32) -> Self {
        Self { refractive_index }
    }

    fn reflectance(&self, cosine: f32, ref_idx: f32) -> f32 {
        // Using Shlick's Approximation for Reflectance.
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)> {
        let refraction_ratio = if rec.front_face { 
            1.0 / self.refractive_index
        } else { 
            self.refractive_index 
        };

        let unit_direction = unit_vector(&r_in.direction());
        let cos_theta = dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let must_reflect = self.reflectance(cos_theta, refraction_ratio) > random_double(0.0, 1.0);
        
        let direction = if cannot_refract || must_reflect {
            reflect(unit_direction, rec.normal)
        } else {
            refract(unit_direction, rec.normal, refraction_ratio)
        };

        Some((Ray::new(rec.p, direction), Colour::new(1.0, 1.0, 1.0)))
    }
}