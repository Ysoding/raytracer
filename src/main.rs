use anyhow::Result;
use raytracer::{
    random_f64, random_f64_range, random_range_vector, random_vector, Camera, Dielectric,
    Lambertian, Metal,
};
use std::env;

use raytracer::{HittableList, Sphere, Vec3};

fn main() -> Result<()> {
    env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();
    // World
    let mut world = HittableList::default();

    let ground_material = Lambertian::new(Vec3::new(0.5, 0.5, 0.5));
    world.objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let chosse_mat = random_f64();
            let center = Vec3::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                // use box
                // let sphere_material: Box<dyn Material> = if chosse_mat < 0.8 {
                //     //diffues
                //     let albedo = random_vector() * random_vector();
                //     Box::new(Lambertian::new(albedo))
                // } else if chosse_mat < 0.95 {
                //     // metal
                //     let albedo = random_range_vector(0.5, 1.0);
                //     let fuzz = random_f64_range(0.0, 0.5);
                //     Box::new(Metal::new(albedo, fuzz))
                // } else {
                //     //glass
                //     Box::new(Dielectric::new(1.5))
                // };
                // world
                //     .objects
                //     .push(Box::new(Sphere::new(center, 0.2, sphere_material)));

                if chosse_mat < 0.8 {
                    let albedo = random_vector() * random_vector();
                    let sphere_material = Lambertian::new(albedo);
                    world
                        .objects
                        .push(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if chosse_mat < 0.95 {
                    let albedo = random_range_vector(0.5, 1.0);
                    let fuzz = random_f64_range(0.0, 0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world
                        .objects
                        .push(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Dielectric::new(1.5);
                    world
                        .objects
                        .push(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Lambertian::new(Vec3::new(0.4, 0.2, 0.1));
    world.objects.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0);
    world.objects.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let mut cam = Camera::default();
    cam.aspect_radio = 16.0 / 9.0;
    cam.image_width = 1200.0;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.look_from = Vec3::new(13.0, 2.0, 3.0);
    cam.look_at = Vec3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(&world)?;
    Ok(())
}
