use std::io;

use log::info;

mod utils;
use utils::*;
mod vec3;
use vec3::*;

fn main() {
    env_logger::init();

    let image_width = 256;
    let image_height = 256;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..image_height {
        info!("Scanlines remaining: {} ", (image_height - j));
        for i in 0..image_width {
            write_color(
                &mut io::stdout(),
                Vec3::new(
                    i as f64 / ((image_width - 1) as f64),
                    j as f64 / ((image_height - 1) as f64),
                    0.0,
                ),
            );
        }
    }
    info!("Done.")
}
