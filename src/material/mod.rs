use crate::{color::Color, hit::HitRecord, ray::Ray, vector::Vector3};

pub struct ScatterRecord {
    pub ray: Ray,
    pub attenuation: Color,
}

pub trait Scatterable {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord>;
}

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Scatterable for Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        match *self {
            Material::Lambertian(ref inner) => inner.scatter(ray, hit),
            Material::Metal(ref inner) => inner.scatter(ray, hit),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Scatterable for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let scatter_direction = hit.normal + Vector3::random_unit_vector();

        let record = ScatterRecord {
            ray: Ray::new(hit.point, scatter_direction),
            attenuation: self.albedo.clone(),
        };

        Some(record)
    }
}

#[derive(Clone, Copy)]
pub struct Metal {
    pub albedo: Color,
}

impl Metal {
    fn reflect(v: Vector3, n: Vector3) -> Vector3 {
        v - 2.0 * (v * n) * n
    }
}

impl Scatterable for Metal {
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
