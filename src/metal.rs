use crate::{
    material::{Material, Scattered},
    ray::Ray,
    vec3::Color,
};

pub struct Metal {
    pub albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &crate::ray::Ray, rec: &crate::hittable::HitRecord) -> Option<Scattered> {
        let reflected = r_in.direction().unit_vector().reflect(rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;

        if scattered.direction().dot(rec.normal) > 0.0 {
            return Some(Scattered { scattered, attenuation });
        } else {
            return None;
        }
    }
}
