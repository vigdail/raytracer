use crate::vector::Vector3;

#[derive(Debug)]
pub struct Ray {
    orig: Vector3,
    dir: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Self {
            orig: origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> Vector3 {
        self.orig
    }

    pub fn direction(&self) -> Vector3 {
        self.dir
    }

    pub fn at(&self, p: f32) -> Vector3 {
        self.orig + p * self.dir
    }
}
