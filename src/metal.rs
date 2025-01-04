use crate::{
    material::{Material, Scattered},
    ray::Ray,
    vec3::{Color, Vec3},
};

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &crate::ray::Ray, rec: &crate::hittable::HitRecord) -> Option<Scattered> {
        let reflected = r_in.direction().unit_vector().reflect(rec.normal);
        let scattered = Ray::new(rec.p, reflected + Vec3::random_in_unit_sphere() * self.fuzz);
        let attenuation = self.albedo;

        if scattered.direction().dot(rec.normal) > 0.0 {
            return Some(Scattered { scattered, attenuation });
        } else {
            return None;
        }
    }
}
