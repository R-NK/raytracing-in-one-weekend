use crate::{
    material::{Material, Scattered},
    math::{random_double, schlick},
    ray::Ray,
    vec3::Color,
};

pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Dielectric { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &crate::ray::Ray, rec: &crate::hittable::HitRecord) -> Option<Scattered> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let etai_over_etat = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = (-unit_direction.dot(rec.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            let reflected = unit_direction.reflect(rec.normal);
            let scattered = Ray::new(rec.p, reflected);

            return Some(Scattered { scattered, attenuation });
        }

        let reflect_prob = schlick(cos_theta, etai_over_etat);
        if random_double() < reflect_prob {
            let reflected = unit_direction.reflect(rec.normal);
            let scattered = Ray::new(rec.p, reflected);

            return Some(Scattered { scattered, attenuation });
        }

        let refracted = unit_direction.reflact(rec.normal, etai_over_etat);
        let scattered = Ray::new(rec.p, refracted);

        Some(Scattered { scattered, attenuation })
    }
}
