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
use std::time::Instant;
use std::sync::Arc;

use rayon::prelude::*;

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


fn random_scene() -> HittableList {
    fn sphere(x: f32, y: f32, z: f32, r: f32, m: Arc<dyn Material>) -> Sphere {
        Sphere::new(Point3::new(x, y, z), r, m)
    }

    let mut world = HittableList::new(vec![]);

    let ground_material = Arc::new(Lambertian::new(Colour::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(sphere(0.0, -1000.0, 0.0, 1000.0, ground_material)));

    for a in -11..11 { 
        for b in -11..11 {
            let (a, b) = (a as f32, b as f32);
            let choose_mat = random_double(0.0, 1.0);
            let centre = Point3::new(a + 0.9 * random_double(0.0, 1.0), 0.2, b + 0.9 * random_double(0.0, 1.0));

            if (centre - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material>;

                if choose_mat < 0.8 {
                    let albedo = Colour::random(0.0, 1.0) * Colour::random(0.0, 1.0);
                    sphere_material = Arc::new(Lambertian::new(albedo));
                } else if choose_mat < 0.95 {
                    let albedo = Colour::random(0.5, 1.0);
                    let fuzz = random_double(0.0, 0.5);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                } else {
                    sphere_material = Arc::new(Dielectric::new(1.5));
                }

                world.add(Arc::new(Sphere::new(centre, 0.2, sphere_material)))
            }
        }
    }
    let dielectric = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(sphere(0.0, 1.0, 0.0, 1.0, dielectric)));

    let lambertian = Arc::new(Lambertian::new(Point3::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(sphere(-4.0, 1.0, 0.0, 1.0, lambertian)));

    let metal = Arc::new(Metal::new(Point3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(sphere(4.0, 1.0, 0.0, 1.0, metal)));

    world
}

fn basic_scene() -> HittableList {
    let mat_ground = Arc::new(Lambertian::new(Colour::new(0.8, 0.8, 0.1)));
    let mat_centre = Arc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.5)));
    let mat_left   = Arc::new(Dielectric::new(1.5));
    let mat_right  = Arc::new(     Metal::new(Colour::new(0.8, 0.6, 0.2), 0.0));

    HittableList::new(vec![
        Arc::new(Sphere::new(Point3::new( 0.0, -100.5, -1.0), 100.0, mat_ground)),
        Arc::new(Sphere::new(Point3::new( 0.0,    0.0, -1.0),   0.5, mat_centre)),
        Arc::new(Sphere::new(Point3::new(-1.0,    0.0, -1.0),   0.5, mat_left.clone())),
        Arc::new(Sphere::new(Point3::new(-1.0,    0.0, -1.0), -0.45, mat_left)),
        Arc::new(Sphere::new(Point3::new( 1.0,    0.0, -1.0),   0.5, mat_right))
    ])
}

fn main() {
    let time_start = Instant::now();
    let file_path = if let Some(fp) = args().nth(1) {
        fp
    } else { panic!("Must provide a path for output.") };

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let mut bmp = Bitmap::new(vec![], image_width);

    // World
    let world = Arc::new(random_scene());

    // Camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_to = Point3::new(0.0, 0.0, 0.0);
    let v_up = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(look_from, look_to, v_up, 20.0, aspect_ratio, aperture, dist_to_focus);

    // Render
    for j in (0..image_height).rev() {
        print!("\r[{:03}%] Rendering Scanline {} of {} {}", ((image_height - j) * 100) / image_height, image_height - j, image_height, throbber(j as usize));
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