mod vec3;
mod colour;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod util;

use std::rc::Rc;

use hittable::HitRecord;

use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::vec3::*;
use crate::colour::*;
use crate::ray::*;
use crate::hittable::Hittable;

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

    // World
    let mut world = HittableList::new(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines Remaining:\t{:03} {}", j, throbber(j as usize));
        for i in 0..image_width {
            let u = i as f32 / (image_width as f32 - 1.0);
            let v = j as f32 / (image_height as f32 - 1.0);
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            let pixel_colour = ray_colour(&r, &world);

            write_colour(&pixel_colour);
        }
    }
    eprintln!("\nDone");
}
