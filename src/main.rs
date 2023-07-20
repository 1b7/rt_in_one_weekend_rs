mod bitmap;
mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod scenes;
mod sphere;
mod stl;
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
    let total_time = Instant::now();
    let file_path = if let Some(fp) = args().nth(1) {
        fp
    } else { panic!("Must provide a path for output.") };

    // Image Settings
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 250;
    let max_depth = 50;

    // World
    print!("[INFO] Loading scene...");
    let world = Arc::new(scenes::custom_model("../teapot.stl"));
    let t_load_model = total_time.elapsed();
    let time_start = Instant::now();
    println!("Done!");

    // Camera
    let look_from = Point3::new(-3.0, -10.0,  8.0);
    let look_to   = Point3::new( 0.0,  0.0,  2.0);
    let dist_to_focus = 10.0;
    let vfov = 70.0;
    let v_up = Vec3::new(0.0, 0.0, 1.0);
    let aperture = 0.1;
    let cam = Camera::new(look_from, look_to, v_up, vfov, aspect_ratio, aperture, dist_to_focus);

    // Render
    println!("[INFO] Beginning render;");
    let mut bmp = Bitmap::new(vec![], image_width);
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
    println!("Done!");
    let t_rendering = time_start.elapsed();

    let time_start = Instant::now();
    print!("[INFO] Writing Image... ");
    let out_file = std::fs::File::create(&file_path).unwrap();
    bmp.output(out_file).unwrap();
    println!("Done!");
    let t_writing = time_start.elapsed();

    let t_total = total_time.elapsed();
    println!("[INFO] Image written to: {}\n\n\
        ==========| Statistics |=========\n\
        Total runtime:       \t{:>8.3}s\n\
        Time to load model:  \t{:>8.3}s\n\
        Time to render image:\t{:>8.3}s\n\
        Time to write image: \t{:>8.3}s\n\
        Shapes in scene:     \t{:>4}",
        
        file_path,
        t_total.as_secs_f32(),
        t_load_model.as_secs_f32(),
        t_rendering.as_secs_f32(),
        t_writing.as_secs_f32(),
        world.len()
    );
}