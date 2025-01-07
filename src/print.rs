use std::io::{self, Write};

use crate::vec3::Color;

pub fn write_color<W: Write>(out: &mut W, pixel_color: Color, samples_per_pixel: u16) -> io::Result<()> {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    writeln!(
        out,
        "{} {} {}",
        (255.999 * clamp(r, 0.0, 0.999)) as i16,
        (255.999 * clamp(g, 0.0, 0.999)) as i16,
        (255.999 * clamp(b, 0.0, 0.999)) as i16
    )
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }

    x
}
