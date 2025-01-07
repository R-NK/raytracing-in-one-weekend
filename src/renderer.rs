use std::{
    f64::INFINITY,
    io::{self, Write},
    rc::Rc,
    time::Instant,
};

use crate::{
    camera::Camera,
    dielectric::Dielectric,
    hittable::{Hittable, HittableList},
    lambertian::Lambertian,
    material::Material,
    math::{random_double, random_double_range},
    metal::Metal,
    print, ray,
    sphere::Sphere,
    vec3::{Color, Point3, Vec3},
};

pub fn render<W: Write>(
    out: &mut W,
    width: u16,
    height: u16,
    samples_per_pixel: u16,
    max_depth: u16,
) -> io::Result<()> {
    let aspect_ratio: f64 = width as f64 / height as f64;

    let world = random_scene();

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, dist_to_focus);

    let start = Instant::now();

    writeln!(out, "P3\n{} {}\n255", width, height)?;

    for j in (0..height).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..width {
            let mut pixel_color = Color::default();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random_double()) / (width - 1) as f64;
                let v = (j as f64 + random_double()) / (height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            print::write_color(out, pixel_color, samples_per_pixel)?;
        }
    }

    let duration = start.elapsed();
    eprintln!("\nDone. Duration: {:?}s", duration.as_secs());

    Ok(())
}

fn ray_color<T: Hittable>(r: &ray::Ray, world: &T, depth: u16) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, INFINITY) {
        if let Some(scattered) = rec.material.scatter(r, &rec) {
            return scattered.1 * ray_color(&scattered.0, world, depth - 1);
        }

        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);

    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn random_scene() -> HittableList {
    let mut world = HittableList::default();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(a as f64 + 0.9 * random_double(), 0.2, b as f64 + 0.9 * random_double());

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material>;
                match choose_mat {
                    n if n > 0.8 => {
                        // diffuse
                        let albedo = Color::random() * Color::random();
                        sphere_material = Rc::new(Lambertian::new(albedo));
                        world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                    }
                    n if n < 0.95 => {
                        // metal
                        let albedo = Color::random_range(0.5, 1.0);
                        let fuzz = random_double_range(0.0, 0.5);
                        sphere_material = Rc::new(Metal::new(albedo, fuzz));
                        world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                    }
                    _ => {
                        // glass
                        sphere_material = Rc::new(Dielectric::new(1.5));
                        world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                    }
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 1.0), 1.0, material2)));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3)));

    world
}
