mod bitmap;
mod camera;
mod colour;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod util;
mod vec3;

use std::env::args;
use std::rc::Rc;

use bitmap::*;
use camera::Camera;
use colour::*;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::*;
use sphere::Sphere;
use util::random_double;
use vec3::*;

fn ray_colour(r: &Ray, world: &impl Hittable, depth: usize) -> Colour {
    let mut rec = HitRecord::default();

    if depth == 0 { return Colour::new(0.0, 0.0, 0.0) }

    if world.hit(r, 0.0, f32::INFINITY, &mut rec) {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        return 0.5 * ray_colour(&Ray::new(rec.p, target - rec.p), world, depth - 1)
    }
    let unit_direction = unit_vector(&r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
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
    let file_path = if let Some(fp) = args().nth(1) {
        fp
    } else { panic!("Must provide a path for output.") };

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let mut bmp = Bitmap::new(vec![], image_width);

    // World
    let mut world = HittableList::new(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let cam = Camera::new();

    // Render
    for j in (0..image_height).rev() {
        eprint!("\rRendering Scanline {} of {} {}", image_height - j, image_height, throbber(j as usize));
        for i in 0..image_width {
            let mut pixel_colour = Colour::default();
            for _ in 0..samples_per_pixel {
                let u = (i as f32 + random_double(0.0, 1.0)) / (image_width - 1) as f32;
                let v = (j as f32 + random_double(0.0, 1.0)) / (image_height - 1) as f32;
                let r = cam.get_ray(u, v);
                pixel_colour += ray_colour(&r, &world, max_depth);
            }

            bmp.push_pixel(col_as_rgb(&pixel_colour, samples_per_pixel));
        }
    }

    eprintln!("Done!");
    
    let out_file = std::fs::File::create(&file_path).unwrap();
    bmp.output(out_file).unwrap();

    eprintln!("Image written to: {}", file_path);
}
