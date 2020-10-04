use crate::hit::{HitRecord, Hittable};

pub struct Scene {
    entities: Vec<Box<dyn Hittable>>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }

    pub fn add(&mut self, entity: Box<dyn Hittable>) {
        self.entities.push(entity);
    }

    pub fn clear(&mut self) {
        self.entities.clear();
    }
}

impl Hittable for Scene {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut result: Option<HitRecord> = None;
        let mut t_closest = t_max;
        self.entities.iter().for_each(|e| {
            if let Some(hit) = e.hit(ray, t_min, t_closest) {
                t_closest = hit.t;
                result = Some(hit);
            }
        });

        result
    }
}
