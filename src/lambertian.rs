use crate::{
    hittable::HitRecord,
    material::{Material, Scattered},
    ray::Ray,
    vec3::{Color, Vec3},
};

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<Scattered> {
        let scatter_direction = rec.normal + Vec3::random_unit_vector();
        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;

        return Some(Scattered { scattered, attenuation });
    }
}
