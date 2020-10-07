use crate::{ray::Ray, util::deg_to_rad, vector::Vector3};

pub struct Camera {
    origin: Vector3,
    left_bottom: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
}

impl Camera {
    pub fn new(
        look_from: Vector3,
        look_at: Vector3,
        vup: Vector3,
        vfov: f32,
        aspect_ratio: f32,
    ) -> Self {
        let theta = deg_to_rad(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_ratio;

        let w = (look_from - look_at).normalized();
        let u = (vup ^ w).normalized();
        let v = w ^ u;

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let left_bottom = origin - horizontal * 0.5 - vertical * 0.5 - w;

        Self {
            origin,
            left_bottom,
            horizontal,
            vertical,
        }
    }

    pub fn ray(&self, s: f32, t: f32) -> Ray {
        Ray::new(
            self.origin,
            self.left_bottom + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}
