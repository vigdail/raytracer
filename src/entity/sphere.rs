use std::rc::Rc;

use crate::{hit::HitRecord, hit::Hittable, material::Material, ray::Ray, vector::Vector3};

pub struct Sphere {
    center: Vector3,
    radius: f32,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f32, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc: Vector3 = ray.origin() - self.center;
        let a = ray.direction().squared_length();
        let half_b = oc * ray.direction();
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant <= 0.0 {
            return None;
        }

        let root = discriminant.sqrt();

        let get_t = || {
            let t1 = (-half_b - root) / a;
            if t1 < t_max && t1 > t_min {
                return Some(t1);
            }
            let t2 = (-half_b + root) / a;
            if t2 < t_max && t2 > t_min {
                return Some(t2);
            }

            None
        };

        let t = get_t()?;
        let hit_point = ray.at(t);
        let outward_normal = (hit_point - self.center) / self.radius;
        let (face, normal) = HitRecord::get_face_normal(ray, outward_normal);
        let record = HitRecord {
            t,
            point: hit_point,
            material: self.material.clone(),
            normal,
            face,
        };

        Some(record)
    }
}
