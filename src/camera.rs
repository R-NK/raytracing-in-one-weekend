use crate::{
    math::degrees_to_radians,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Point3,
    vertical: Point3,
    lens_radius: f64,
    u: Vec3,
    v: Vec3,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;
        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            lens_radius,
            u,
            v,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
        )
    }
}
