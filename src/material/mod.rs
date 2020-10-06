use std::fmt::Debug;

use crate::{color::Color, hit::HitRecord, ray::Ray, vector::Vector3};

#[derive(Debug)]
pub struct ScatterRecord {
    pub ray: Ray,
    pub attenuation: Color,
}

pub trait Material: Debug {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord>;
}

#[derive(Debug, Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let scatter_direction = hit.normal + Vector3::random_unit_vector();

        let record = ScatterRecord {
            ray: Ray::new(hit.point, scatter_direction),
            attenuation: self.albedo.clone(),
        };

        Some(record)
    }
}

#[derive(Debug)]
pub struct Metal {
    pub albedo: Color,
}

impl Metal {
    fn reflect(v: Vector3, n: Vector3) -> Vector3 {
        v - 2.0 * (v * n) * n
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let reflected = Metal::reflect(ray.direction(), hit.normal);
        let scattered = Ray::new(hit.point, reflected);

        if scattered.direction() * hit.normal > 0.0 {
            let record = ScatterRecord {
                ray: scattered,
                attenuation: self.albedo.clone(),
            };
            Some(record)
        } else {
            None
        }
    }
}
