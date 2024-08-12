use std::io::Write;

use anyhow::Result;

use super::vec3::Vec3;

pub fn write_color(out: &mut impl Write, color: Vec3) -> Result<()> {
    let ir = (255.999 * color.x) as i32;
    let ig = (255.999 * color.y) as i32;
    let ib = (255.999 * color.z) as i32;

    writeln!(out, "{} {} {}", ir, ig, ib)?;
    Ok(())
}
