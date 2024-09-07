use std::io::Write;

use anyhow::Result;
use rand::Rng;

use crate::Interval;

use super::vec3::Vec3;

pub fn write_color(out: &mut impl Write, color: Vec3) -> Result<()> {
    // Translate the [0,1] component values to the byte range [0,255].
    let r = linear_to_gamma(color.x);
    let g = linear_to_gamma(color.y);
    let b = linear_to_gamma(color.z);

    let intensity = Interval::new(0.000, 0.999);
    let ir = (255.0 * intensity.clamp(r)) as i32;
    let ig = (255.0 * intensity.clamp(g)) as i32;
    let ib = (255.0 * intensity.clamp(b)) as i32;

    writeln!(out, "{} {} {}", ir, ig, ib)?;
    Ok(())
}

pub fn random_f64_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn random_uint_vector() -> Vec3 {
    loop {
        let p = random_range_vector(-1.0, 1.0);
        let lensq = p.length_squard();
        if 1e-160 < lensq && lensq <= 1.0 {
            return p / lensq.sqrt();
        }
    }
}

pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let on_uint_sphere = random_uint_vector();
    if on_uint_sphere.dot(normal) > 0.0 {
        on_uint_sphere
    } else {
        -on_uint_sphere
    }
}

pub fn random_range_vector(min: f64, max: f64) -> Vec3 {
    Vec3 {
        x: random_f64_range(min, max),
        y: random_f64_range(min, max),
        z: random_f64_range(min, max),
    }
}

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}
