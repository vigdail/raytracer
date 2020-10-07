use crate::{color::Color, hit::Face, hit::HitRecord, ray::Ray, util::Random, vector::Vector3};

pub struct ScatterRecord {
    pub ray: Ray,
    pub attenuation: Color,
}

pub trait Scatterable {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord>;

    fn reflect(v: Vector3, n: Vector3) -> Vector3 {
        v - 2.0 * (v * n) * n
    }
}

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Scatterable for Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        match *self {
            Material::Lambertian(ref inner) => inner.scatter(ray, hit),
            Material::Metal(ref inner) => inner.scatter(ray, hit),
            Material::Dielectric(ref inner) => inner.scatter(ray, hit),
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
            attenuation: self.albedo,
        };

        Some(record)
    }
}

#[derive(Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, mut fuzz: f32) -> Metal {
        if fuzz > 1.0 {
            fuzz = 1.0;
        }
        if fuzz < 0.0 {
            fuzz = 0.0;
        }
        Metal { albedo, fuzz }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let reflected = Metal::reflect(ray.direction(), hit.normal);
        let scattered = Ray::new(
            hit.point,
            reflected + self.fuzz * Vector3::random_in_unit_sphere(),
        );

        if scattered.direction() * hit.normal > 0.0 {
            let record = ScatterRecord {
                ray: scattered,
                attenuation: self.albedo,
            };
            Some(record)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
pub struct Dielectric {
    ir: f32,
}

impl Dielectric {
    pub fn new(ir: f32) -> Dielectric {
        Dielectric { ir }
    }

    fn refract(uv: Vector3, n: Vector3, i: f32) -> Vector3 {
        let cos_theta = -uv * n;

        let perp: Vector3 = i * (uv + cos_theta * n);
        let par = -(1.0 - perp.squared_length()).abs().sqrt() * n;

        perp + par
    }

    fn refractance(cos: f32, refr_idx: f32) -> f32 {
        let r0 = (1.0 - refr_idx) / (1.0 + refr_idx);
        let r0 = r0 * r0;

        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

impl Scatterable for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let ratio = match hit.face {
            Face::Front => 1.0 / self.ir,
            Face::Back => self.ir,
        };
        let unit_dir = ray.direction().normalized();
        let cos_theta = (-unit_dir * hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction = if sin_theta * ratio > 1.0
            || Dielectric::refractance(cos_theta, ratio) > f32::random()
        {
            Dielectric::reflect(unit_dir, hit.normal)
        } else {
            Dielectric::refract(unit_dir, hit.normal, ratio)
        };

        let scattered = Ray::new(hit.point, direction);

        Some(ScatterRecord {
            attenuation: Color::rgb(1.0, 1.0, 1.0),
            ray: scattered,
        })
    }
}
