use crate::{ray::Ray, util::deg_to_rad, vector::Vector3};

#[derive(Debug)]
pub struct Camera {
    origin: Vector3,
    left_bottom: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    w: Vector3,
    u: Vector3,
    v: Vector3,
    lens_radius: f32,
    focus_dist: f32,
}

impl Camera {
    pub fn new(
        look_from: Vector3,
        look_at: Vector3,
        vup: Vector3,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let theta = deg_to_rad(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_ratio;

        let w = (look_from - look_at).normalized();
        let u = (vup ^ w).normalized();
        let v = w ^ u;

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let left_bottom = origin - horizontal * 0.5 - vertical * 0.5 - focus_dist * w;

        Self {
            origin,
            left_bottom,
            horizontal,
            vertical,
            w,
            u,
            v,
            lens_radius: aperture / 2.0,
            focus_dist,
        }
    }

    pub fn ray(&self, s: f32, t: f32) -> Ray {
        let rd = Vector3::random_unit_vector() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.left_bottom + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
