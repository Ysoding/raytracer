use std::io;

use anyhow::Result;
use log::info;

use crate::{write_color, Hittable, Interval, Ray, Vec3};

pub struct Camera {
    pub aspect_radio: f64, // Ratio of image width over height
    pub image_width: f64,  // Rendered image width in pixel count
    image_height: f64,     // Rendered image height
    center: Vec3,          // Camera center
    pixel00_loc: Vec3,     // Location of pixel 0, 0
    pixel_delta_u: Vec3,   // Offset to pixel to the right
    pixel_delta_v: Vec3,   // Offset to pixel below
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_radio: 1.0,
            image_width: 100.0,
            image_height: Default::default(),
            center: Default::default(),
            pixel00_loc: Default::default(),
            pixel_delta_u: Default::default(),
            pixel_delta_v: Default::default(),
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
                let pixel_center = self.pixel00_loc
                    + (self.pixel_delta_u * i as f64)
                    + (self.pixel_delta_v * j as f64);
                let ray_directionection = pixel_center - self.center;
                let r = Ray::new(self.center, ray_directionection);

                let pixel_color = self.ray_color(&r, world);

                write_color(&mut io::stdout(), pixel_color)?;
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

    fn ray_color(&self, ray: &Ray, world: &dyn Hittable) -> Vec3 {
        if let Some(hr) = world.hit(ray, Interval::new(0.0, f64::INFINITY)) {
            return (hr.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
        }

        let unit_directionection = ray.direction.unit();
        let a = 0.5 * (unit_directionection.y + 1.0);
        Vec3::ones() * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
    }
}
