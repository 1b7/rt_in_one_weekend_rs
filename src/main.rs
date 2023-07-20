mod bitmap;
mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod scenes;
mod sphere;
mod triangle;
mod util;
mod vec3;

use std::{env::args, io::Write};
use std::time::Instant;
use std::sync::Arc;

use rayon::prelude::*;

use bitmap::*;
use camera::Camera;
use hittable::Hittable;
use hittable_list::HittableList;
use ray::*;
use util::*;
use vec3::*;


fn ray_colour(r: &Ray, world: &Arc<HittableList>, depth: usize) -> Colour {
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
    let time_start = Instant::now();
    let file_path = if let Some(fp) = args().nth(1) {
        fp
    } else { panic!("Must provide a path for output.") };

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1920;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 500;
    let max_depth = 100;

    let mut bmp = Bitmap::new(vec![], image_width);

    // World
    let world = Arc::new(scenes::cornell());

    // Camera
    let look_from = Point3::new(1.5, 1.3,  2.0);
    let look_to   = Point3::new(1.5, 0.9, -1.0);
    let dist_to_focus = 3.5;
    let vfov = 30.0;
    let v_up = Vec3::new(0.0, 1.0, 0.0);
    let aperture = 0.1;
    let cam = Camera::new(look_from, look_to, v_up, vfov, aspect_ratio, aperture, dist_to_focus);

    // Render
    for j in (0..image_height).rev() {
        print!("\r[{:>3}%] Rendering Scanline {} of {} {}", ((image_height - j) * 100) / image_height, image_height - j, image_height, throbber(j as usize));
        let _ = std::io::stdout().flush();
        let pixels = (0..image_width).into_par_iter().map(|i| {
            let mut pixel_colour = Colour::default();
            for _ in 0..samples_per_pixel {
                let u = (i as f32 + random_double(0.0, 1.0)) / (image_width - 1) as f32;
                let v = (j as f32 + random_double(0.0, 1.0)) / (image_height - 1) as f32;
                let r = cam.get_ray(u, v);
                pixel_colour += ray_colour(&r, &world, max_depth);
            }
           col_as_rgb(&pixel_colour, samples_per_pixel)
        }).collect::<Vec<_>>();
        bmp.push_slice(&pixels)
    }
    let elapsed = time_start.elapsed();
    println!("Done!");

    print!("Writing Image... ");
    let out_file = std::fs::File::create(&file_path).unwrap();
    bmp.output(out_file).unwrap();
    println!("Done!");
    println!("Image written to: {}", file_path);
    println!("Render took: {:.3}s", elapsed.as_secs_f64());
}