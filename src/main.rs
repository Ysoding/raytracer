use anyhow::Result;
use raytracer::{Camera, Dielectric, Lambertian, Metal};
use std::env;

use raytracer::{HittableList, Sphere, Vec3};

fn main() -> Result<()> {
    env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();
    // World
    let mut world = HittableList::default();

    let material_ground = Lambertian::new(Vec3::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Vec3::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.50);
    let material_bubble = Dielectric::new(1.00 / 1.50);
    let material_right = Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0);

    world.objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    world.objects.push(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.objects.push(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    )));
    world.objects.push(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let mut camera = Camera::default();
    camera.aspect_radio = 16.0 / 9.0;
    camera.image_width = 400.0;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.render(&world)?;
    Ok(())
}
