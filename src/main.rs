use anyhow::Result;
use raytracer::Camera;
use std::env;

use raytracer::{HittableList, Sphere, Vec3};

fn main() -> Result<()> {
    env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();
    // World
    let mut world = HittableList::default();
    world
        .objects
        .push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world
        .objects
        .push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let mut camera = Camera::default();
    camera.aspect_radio = 16.0 / 9.0;
    camera.image_width = 400.0;

    camera.render(&world)?;
    Ok(())
}
