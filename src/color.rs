use std::io::Write;

use crate::{geometry::vec3::Vec3, util::SAMPLES_PER_PIXEL};

// Type alias
pub type Color = Vec3;

pub fn write_color(out: &mut impl Write, pixel_color: Color) {
    // Write the translated [0, 255] value of each color component
    let mut r = pixel_color.x();
    let mut b = pixel_color.z();
    let mut g = pixel_color.y();

    // divide color by number of samples to for accumlation of light and gamma-correct for gamma=2.0
    let scale = 1.0 / SAMPLES_PER_PIXEL as f64;
    r = f64::sqrt(scale * r);
    g = f64::sqrt(scale * g);
    b = f64::sqrt(scale * b);

    writeln!(
        out,
        "{} {} {}",
        (256.0 * r.clamp(0.0, 0.999)) as i32,
        (256.0 * g.clamp(0.0, 0.999)) as i32,
        (256.0 * b.clamp(0.0, 0.999)) as i32,
    )
    .expect("writing color");
}
