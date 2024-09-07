use anyhow::Result;
use log::info;
use std::env;
use std::io;

use raytracer::hittable::*;
use raytracer::ray::*;
use raytracer::sphere::*;
use raytracer::utils::*;
use raytracer::vec3::*;

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Vec3 {
    if let Some(hr) = world.hit(ray, 0.0, f64::INFINITY) {
        return (hr.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
    }

    let unit_directionection = ray.direction.unit();
    let a = 0.5 * (unit_directionection.y + 1.0);
    Vec3::ones() * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
}

fn main() -> Result<()> {
    env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    // Image
    let image_width = 400.0;
    let aspect_ratio = 16.0 / 9.0;

    // Calculate the image height, and ensure that it's at least 1.
    let mut image_height = image_width / aspect_ratio;
    image_height = if image_height < 1.0 {
        1.0
    } else {
        image_height
    };
    // World
    let mut world = HittableList::default();
    world
        .objects
        .push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world
        .objects
        .push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width / image_height);
    let camera_center = Vec3::zero();

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width;
    let pixel_delta_v = viewport_v / image_height;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    // Render
    for j in 0..image_height as i32 {
        info!("Scanlines remaining: {} ", (image_height as i32 - j));
        for i in 0..image_width as i32 {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_directionection = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_directionection);

            let pixel_color = ray_color(&r, &world);

            write_color(&mut io::stdout(), pixel_color)?;
        }
    }
    info!("Done.");
    Ok(())
}
