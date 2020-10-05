use crate::{ray::Ray, vector::Vector3};

pub struct Camera {
    origin: Vector3,
    left_bottom: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;
        let focal_length = 1.0;

        let origin = Vector3::new();
        let horizontal = Vector3::xyz(viewport_width, 0.0, 0.0);
        let vertical = Vector3::xyz(0.0, viewport_height, 0.0);
        let left_bottom =
            origin - horizontal * 0.5 - vertical * 0.5 - Vector3::xyz(0.0, 0.0, focal_length);

        Self {
            origin,
            left_bottom,
            horizontal,
            vertical,
        }
    }

    pub fn ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.left_bottom + u * self.horizontal + v * self.vertical,
        )
    }
}
