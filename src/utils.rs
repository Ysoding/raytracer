use std::io::Write;

use crate::vec3::*;

pub fn write_color(out: &mut impl Write, color: Vec3) {
    let ir = (255.999 * color.x) as i32;
    let ig = (255.999 * color.y) as i32;
    let ib = (255.999 * color.z) as i32;

    writeln!(out, "{} {} {}", ir, ig, ib);
}
