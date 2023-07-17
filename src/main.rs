mod vec3;
mod colour;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod util;
mod camera;

use std::rc::Rc;

use camera::Camera;
use colour::*;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::*;
use sphere::Sphere;
use util::random_double;
use vec3::*;

fn ray_colour(r: &Ray, world: &impl Hittable) -> Colour {
    let mut rec = HitRecord::default();
    if world.hit(r, 0.0, f32::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Colour::new(1.0, 1.0, 1.0))
    }
    let unit_direction = unit_vector(&r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
}

fn hit_sphere(centre: &Point3, radius: f32, r: &Ray) -> f32 {
    let oc = r.origin() - *centre;
    let a = r.direction().length_squared();
    let half_b = dot(&oc, &r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn throbber(x: usize) -> &'static str {
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

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 100;

    // World
    let mut world = HittableList::new(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let cam = Camera::new();

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines Remaining:\t{:03} {}", j, throbber(j as usize));
        for i in 0..image_width {
            let mut pixel_colour = Colour::default();
            for _ in 0..samples_per_pixel {
                let u = (i as f32 + random_double(0.0, 1.0)) / (image_width - 1) as f32;
                let v = (j as f32 + random_double(0.0, 1.0)) / (image_height - 1) as f32;
                let r = cam.get_ray(u, v);
                pixel_colour += ray_colour(&r, &world);
            }

            write_colour(&pixel_colour, samples_per_pixel);
        }
    }
    eprintln!("\nDone");
}
