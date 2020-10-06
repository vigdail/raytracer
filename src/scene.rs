use crate::{
    entity::Entity,
    hit::{HitRecord, Hittable},
    ray::Ray,
};

pub struct Scene {
    entities: Vec<Entity>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }

    pub fn add(&mut self, entity: Entity) {
        self.entities.push(entity);
    }
}

impl Hittable for Scene {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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
