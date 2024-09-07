use std::io;

use anyhow::Result;
use log::info;

use crate::{random_f64_range, write_color, Hittable, Interval, Ray, Vec3};

pub struct Camera {
    pub aspect_radio: f64,      // Ratio of image width over height
    pub image_width: f64,       // Rendered image width in pixel count
    pub samples_per_pixel: i32, // Count of random samples for each pixel
    pub max_depth: i32,         // Maximum number of ray bounces into scene
    image_height: f64,          // Rendered image height
    center: Vec3,               // Camera center
    pixel00_loc: Vec3,          // Location of pixel 0, 0
    pixel_delta_u: Vec3,        // Offset to pixel to the right
    pixel_delta_v: Vec3,        // Offset to pixel below
    pixel_samples_scale: f64,   // Color scale factor for a sum of pixel samples
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_radio: 1.0,
            image_width: 100.0,
            samples_per_pixel: 10,
            max_depth: 10,
            image_height: Default::default(),
            center: Default::default(),
            pixel00_loc: Default::default(),
            pixel_delta_u: Default::default(),
            pixel_delta_v: Default::default(),
            pixel_samples_scale: Default::default(),
        }
    }
}

impl Camera {
    pub fn render(&mut self, world: &dyn Hittable) -> Result<()> {
        self.initialize();

        info!("Start Render.");

        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        for j in 0..self.image_height as i32 {
            info!("Scanlines remaining: {} ", (self.image_height as i32 - j));
            for i in 0..self.image_width as i32 {
                let mut pixel_color = Vec3::zero();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += self.ray_color(&ray, world, self.max_depth);
                }

                write_color(&mut io::stdout(), pixel_color * self.pixel_samples_scale)?;
            }
        }
        info!("Done.");
        Ok(())
    }

    fn initialize(&mut self) {
        // Calculate the image height, and ensure that it's at least 1.
        self.image_height = self.image_width / self.aspect_radio;
        self.image_height = if self.image_height < 1.0 {
            1.0
        } else {
            self.image_height
        };
        self.center = Vec3::zero();
        self.pixel_samples_scale = 1.0 / (self.samples_per_pixel as f64);

        // Determine viewport dimensions.
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width / self.image_height);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width;
        self.pixel_delta_v = viewport_v / self.image_height;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
    }

    fn ray_color(&self, ray: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Vec3::zero();
        }

        if let Some(hr) = world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
            if let Some(sr) = hr.material.scatter(ray, &hr) {
                return sr.attenuation * self.ray_color(&sr.scatter_ray, world, depth - 1);
            }
            return Vec3::zero();
        }

        let unit_directionection = ray.direction.unit();
        let a = 0.5 * (unit_directionection.y + 1.0);
        Vec3::ones() * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location i, j.
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);

        Ray::new(self.center, pixel_sample - self.center)
    }

    fn sample_square(&self) -> Vec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        Vec3::new(
            random_f64_range(0.0, 1.0) - 0.5,
            random_f64_range(0.0, 1.0) - 0.5,
            0.0,
        )
    }
}
