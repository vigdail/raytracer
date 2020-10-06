use std::rc::Rc;

use crate::{material::Material, ray::Ray, vector::Vector3};

#[derive(Debug, Clone)]
pub enum Face {
    Front,
    Back,
}

impl Default for Face {
    fn default() -> Self {
        Face::Front
    }
}

#[derive(Debug)]
pub struct HitRecord {
    pub point: Vector3,
    pub normal: Vector3,
    pub material: Rc<Box<dyn Material>>,
    pub t: f32,
    pub face: Face,
}

impl HitRecord {
    pub fn get_face_normal(ray: &Ray, outward_normal: Vector3) -> (Face, Vector3) {
        let is_frontface = ray.direction() * outward_normal < 0.0;
        let normal = if is_frontface {
            outward_normal
        } else {
            -outward_normal
        };

        let face = if is_frontface {
            Face::Front
        } else {
            Face::Back
        };

        (face, normal)
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
