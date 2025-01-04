use std::rc::Rc;

use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    vec3::Vec3,
};

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Rc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let mut temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                let mut rec = HitRecord {
                    p: r.at(temp),
                    t: temp,
                    normal: Vec3::default(),
                    material: self.material.clone(),
                    front_face: false,
                };
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);

                return Some(rec);
            }

            temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                let mut rec = HitRecord {
                    p: r.at(temp),
                    t: temp,
                    normal: Vec3::default(),
                    material: self.material.clone(),
                    front_face: false,
                };
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);

                return Some(rec);
            }
        }

        None
    }
}
