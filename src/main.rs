mod bitmap;
mod camera;
mod colour;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod util;
mod vec3;

use std::{env::args, io::Write};
use std::rc::Rc;

use bitmap::*;
use camera::Camera;
use colour::*;
use hittable::Hittable;
use hittable_list::HittableList;
use material::*;
use ray::*;
use sphere::Sphere;
use util::*;
use vec3::*;

fn ray_colour(r: &Ray, world: &impl Hittable, depth: usize) -> Colour {
    if depth == 0 { return Colour::new(0.0, 0.0, 0.0) }

    if let Some(hit_record) = world.hit(r, 0.001, f32::INFINITY) {
        return if let Some((scattered, attenuation)) = hit_record.material.scatter(r, &hit_record) {
             attenuation * ray_colour(&scattered, world, depth - 1)
        } else { Colour::default() }
    }

    let unit_direction = unit_vector(&r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
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
    let mat_ground = Rc::new(Lambertian::new(Colour::new(0.8, 0.8, 0.1)));
    let mat_centre = Rc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.5)));
    let mat_left   = Rc::new(Dielectric::new(1.5));
    let mat_right  = Rc::new(     Metal::new(Colour::new(0.8, 0.6, 0.2), 0.0));

    let world = HittableList::new(vec![
        Rc::new(Sphere::new(Point3::new( 0.0, -100.5, -1.0), 100.0, mat_ground)),
        Rc::new(Sphere::new(Point3::new( 0.0,    0.0, -1.0),   0.5, mat_centre)),
        Rc::new(Sphere::new(Point3::new(-1.0,    0.0, -1.0),   0.5, mat_left.clone())),
        Rc::new(Sphere::new(Point3::new(-1.0,    0.0, -1.0), -0.45, mat_left)),
        Rc::new(Sphere::new(Point3::new( 1.0,    0.0, -1.0),   0.5, mat_right))
    ]);

    // Camera
    let cam = Camera::new(Point3::new(0.0, 0.0, 1.0), Point3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 1.0, 0.0), 70.0, aspect_ratio);

    // Render
    for j in (0..image_height).rev() {
        print!("\rRendering Scanline {} of {} {}", image_height - j, image_height, throbber(j as usize));
        let _ = std::io::stdout().flush();
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
    println!("Done!");

    print!("Writing Image... ");
    let out_file = std::fs::File::create(&file_path).unwrap();
    bmp.output(out_file).unwrap();
    println!("Done!");
    println!("Image written to: {}", file_path);
}
