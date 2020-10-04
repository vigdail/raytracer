use crate::vector::Vector3;

pub struct Ray {
    origin: Vector3,
    direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> Vector3 {
        self.origin
    }

    pub fn direction(&self) -> Vector3 {
        self.direction
    }

    pub fn point_at_parameter(&self, p: f32) -> Vector3 {
        self.origin + p * self.direction
    }
}
