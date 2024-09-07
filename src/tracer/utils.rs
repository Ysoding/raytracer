use std::io::Write;

use anyhow::Result;
use rand::Rng;

use crate::Interval;

use super::vec3::Vec3;

pub fn write_color(out: &mut impl Write, color: Vec3) -> Result<()> {
    // Translate the [0,1] component values to the byte range [0,255].
    let intensity = Interval::new(0.000, 0.999);
    let ir = (255.0 * intensity.clamp(color.x)) as i32;
    let ig = (255.0 * intensity.clamp(color.y)) as i32;
    let ib = (255.0 * intensity.clamp(color.z)) as i32;

    writeln!(out, "{} {} {}", ir, ig, ib)?;
    Ok(())
}

pub fn random_f64_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}
