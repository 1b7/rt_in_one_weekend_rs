use std::sync::Arc;

use crate::{hittable_list::HittableList, material::*, sphere::Sphere, triangle::Triangle, vec3::{Point3, Colour}, util::random_double, stl::import};


pub fn random_scene() -> HittableList {
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

pub fn basic_scene() -> HittableList {
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

pub fn cornell() -> HittableList {
    let red = Arc::new(Lambertian::new(Colour::new(0.8, 0.1, 0.1)));
    let green = Arc::new(Lambertian::new(Colour::new(0.1, 0.8, 0.1)));
    let white = Arc::new(Lambertian::new(Colour::new(0.8, 0.8, 0.8)));

    let mat_glass = Arc::new(Dielectric::new(0.8));
    let mat_lamb = Arc::new(Lambertian::new(Colour::new(0.8, 0.8, 0.2)));
    let mat_metal = Arc::new(Metal::new(Colour::new(0.8, 0.2, 0.6), 0.1));

    const R: f32 = 0.4;

    HittableList::new(vec![
        // Left Wall:
        Arc::new(
            Triangle::new(
                Point3::new( 0.0,  0.0, -1.0),
                Point3::new( 0.0,  0.0, -3.0),
                Point3::new( 0.0,  2.0, -1.0),
                red.clone()
            )
        ),
        Arc::new(
            Triangle::new(
                Point3::new( 0.0,  2.0, -1.0),
                Point3::new( 0.0,  0.0, -3.0),
                Point3::new( 0.0,  2.0, -3.0),
                red
            )
        ),

        // Right Wall:
        Arc::new(
            Triangle::new(
                Point3::new( 3.0,  0.0, -1.0),
                Point3::new( 3.0,  2.0, -1.0),
                Point3::new( 3.0,  0.0, -3.0),
                green.clone()
            )
        ),
        Arc::new(
            Triangle::new(
                Point3::new( 3.0,  2.0, -1.0),
                Point3::new( 3.0,  2.0, -3.0),
                Point3::new( 3.0,  0.0, -3.0),
                green
            )
        ),

        // Ground:
        Arc::new(
            Triangle::new(
                Point3::new( 0.0,  0.0, -1.0),
                Point3::new( 3.0,  0.0, -1.0),
                Point3::new( 3.0,  0.0, -3.0),
                white.clone()
            )
        ),
        Arc::new(
            Triangle::new(
                Point3::new( 3.0,  0.0, -3.0),
                Point3::new( 0.0,  0.0, -3.0),
                Point3::new( 0.0,  0.0, -1.0),
                white.clone()
            )
        ),

        // Back Wall:
        Arc::new(
            Triangle::new(
                Point3::new( 3.0,  2.0, -3.0),
                Point3::new( 0.0,  2.0, -3.0),
                Point3::new( 0.0,  0.0, -3.0),
                white.clone()
            )
        ),
        Arc::new(
            Triangle::new(
                Point3::new( 3.0,  2.0, -3.0),
                Point3::new( 0.0,  0.0, -3.0),
                Point3::new( 3.0,  0.0, -3.0),
                white.clone()
            )
        ),

        // Ceiling:
        // Arc::new(
        //     Triangle::new(
        //         Point3::new( 3.0,  2.0, -3.0),
        //         Point3::new( 3.0,  2.0, -1.0),
        //         Point3::new( 0.0,  2.0, -1.0),
        //         white.clone()
        //     )
        // ),
        // Arc::new(
        //     Triangle::new(
        //         Point3::new( 0.0,  2.0, -1.0),
        //         Point3::new( 0.0,  2.0, -3.0),
        //         Point3::new( 3.0,  2.0, -3.0),
        //         white.clone()
        //     )
        // ),

        // Hollow Glass Sphere:
        Arc::new(Sphere::new(Point3::new(1.5, R, -1.4), R, mat_glass.clone())),
        Arc::new(Sphere::new(Point3::new(1.5, R, -1.4), -0.38, mat_glass)),
        // Other Spheres:
        Arc::new(Sphere::new(Point3::new(0.8, R, -2.0), R, mat_metal)),
        Arc::new(Sphere::new(Point3::new(2.2, R, -2.0), R, mat_lamb)),
    ])

}

pub fn basic_scene_tri() -> HittableList {
    let mat_ground = Arc::new(Lambertian::new(Colour::new(0.4, 0.6, 0.3)));
    let mat_orb = Arc::new(Lambertian::new(Colour::new(0.9, 0.6, 0.4)));
    let mat_glass = Arc::new(Dielectric::new(1.5));
    let mat_metal_white = Arc::new(Metal::new(Colour::new(0.8, 0.8, 0.8), 0.02));
    let mat_metal_red = Arc::new(Metal::new(Colour::new(0.8, 0.2, 0.2), 0.05));

    HittableList::new(vec![
        Arc::new(Sphere::new(Point3::new( 0.0, -100.5, -1.0), 100.0, mat_ground)),
        Arc::new(Sphere::new(Point3::new( 0.0, 15.0, -50.0), 20.0, mat_orb)),
        Arc::new(Sphere::new(Point3::new( 0.0, 0.75, -1.0), 0.50, mat_metal_white)),
        Arc::new(
            Triangle::new(
                Point3::new( 0.0,  0.0, -1.0),
                Point3::new( 2.0,  0.0, -1.0),
                Point3::new( 0.0,  2.0, -1.0),
                mat_glass.clone()
            )
        ),
        Arc::new(
            Triangle::new(
                Point3::new( 2.0,  2.0, -1.0), // b
                Point3::new( 0.0,  2.0, -1.0), // c
                Point3::new( 2.0,  0.0, -1.0), // a
                mat_metal_red.clone()
            ),
        ),
        Arc::new(
            Triangle::new(
                Point3::new(-2.0,  2.0, -1.0), // a
                Point3::new(-2.0,  0.0, -1.0), // c
                Point3::new( 0.0,  2.0, -1.0), // b
                mat_metal_red.clone()
            ),
        ),
        Arc::new(
            Triangle::new(
                Point3::new(-2.0,  0.0, -1.0), // a
                Point3::new( 0.0,  0.0, -1.0), // c
                Point3::new( 0.0,  2.0, -1.0), // b
                mat_glass.clone()
            )
        ),

    ])
}

pub fn custom_model(file_path: &str) -> HittableList {
    let raw_tris = import(file_path).unwrap();

    let mat_model = Arc::new(Metal::new(Colour::new(0.8, 0.2, 0.2), 0.01)); 
    let mat_ground = Arc::new(Metal::new(Colour::new(0.6, 0.6, 0.6), 0.05));

    let mut world = HittableList::new(vec![]);
    raw_tris.into_iter().for_each(|tri| world.add(
        Arc::new(Triangle::new(tri[0], tri[1], tri[2], mat_model.clone()))
    ));

    // Add a ground plane:
    world.add(
        Arc::new(Triangle::new(
            Point3::new( -100000.0,  10000.000000, 0.0),
            Point3::new( -100000.0, -10000.000000, 0.0),
            Point3::new(  100000.0, -10000.000000, 0.0),
            mat_ground.clone()
        ))
    );
    
    world.add(
        Arc::new(Triangle::new(
            Point3::new( -100000.0,  10000.000000, 0.0),
            Point3::new(  100000.0, -10000.000000, 0.0),
            Point3::new(  100000.0,  10000.000000, 0.0),
            mat_ground.clone()
        ))
    );
    
    world
}