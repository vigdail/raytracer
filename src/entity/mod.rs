use crate::{hit::Hittable, ray::Ray};

use self::sphere::Sphere;

pub mod sphere;

pub enum Entity {
    Sphere(Sphere),
}

impl Hittable for Entity {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<crate::hit::HitRecord> {
        match *self {
            Entity::Sphere(ref inner) => inner.hit(ray, t_min, t_max),
        }
    }
}
