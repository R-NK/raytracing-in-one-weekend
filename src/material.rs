use crate::{hittable::HitRecord, ray::Ray, vec3::Color};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scattered>;
}

pub struct Scattered {
    pub scattered: Ray,
    pub attenuation: Color,
}
