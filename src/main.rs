use std::io::{self};

use renderer::render;

mod camera;
mod dielectric;
mod hittable;
mod lambertian;
mod material;
mod math;
mod metal;
mod print;
mod ray;
mod renderer;
mod sphere;
mod vec3;

fn main() -> io::Result<()> {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u16 = 768;
    const IMAGE_HEIGHT: u16 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u16;
    const SAMPLES_PER_PIXEL: u16 = 10;
    const MAX_DEPTH: u16 = 50;

    let stdout = std::io::stdout();
    let mut handle = stdout.lock();

    render(&mut handle, IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL, MAX_DEPTH)
}
