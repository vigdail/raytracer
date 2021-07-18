use std::time::SystemTime;

use camera::Camera;
use canvas::Canvas;
use color::Color;
use hit::Hittable;
use material::Scatterable;
use ray::Ray;
use scene::Scene;
use util::Random;
use vector::Vector3;

pub mod camera;
pub mod canvas;
pub mod color;
pub mod entity;
pub mod hit;
pub mod material;
pub mod ray;
pub mod scene;
pub mod util;
pub mod vector;

pub struct Raytracer<'a, T: Canvas> {
    canvas: &'a mut T,
}

impl<'a, T: Canvas> Raytracer<'a, T> {
    pub fn new(canvas: &'a mut T) -> Raytracer<T> {
        Raytracer { canvas }
    }

    pub fn render(&mut self, scene: &Scene) {
        self.draw_scene(scene);
    }

    fn draw_scene(&mut self, scene: &Scene) {
        let width = self.canvas.width();
        let height = self.canvas.height();

        let look_from = Vector3::xyz(13.0, 2.0, 3.0);
        let look_at = Vector3::xyz(0.0, 0.0, 0.0);
        let dist_to_focus = (look_from - look_at).length();

        let camera = Camera::new(
            look_from,
            look_at,
            Vector3::xyz(0.0, 1.0, 0.0),
            100.0,
            width as f32 / height as f32,
            0.1,
            dist_to_focus,
        );

        let samples = 20;
        let max_depth = 50;

        let now = SystemTime::now();

        for j in 0..height {
            for i in 0..width {
                let mut color = Color::new();
                for _ in 0..samples {
                    let di = f32::random();
                    let dj = f32::random();
                    let u = (i as f32 + di) / width as f32;
                    let v = (j as f32 + dj) / height as f32;
                    let ray = camera.ray(u, v);
                    color += self.ray_color(&ray, &scene, max_depth);
                }

                color.r = (color.r / samples as f32).sqrt();
                color.g = (color.g / samples as f32).sqrt();
                color.b = (color.b / samples as f32).sqrt();
                self.canvas.draw_point(&color, i, height - j - 1);
            }
        }

        println!("Done: {} ms", now.elapsed().unwrap().as_millis());
    }

    fn ray_color(&mut self, ray: &Ray, scene: &Scene, depth: u32) -> Color {
        if depth == 0 {
            return Color::new();
        }

        if let Some(hit) = scene.hit(ray, 0.001, f32::INFINITY) {
            if let Some(scatter) = hit.material.scatter(ray, &hit) {
                return scatter.attenuation * self.ray_color(&scatter.ray, scene, depth - 1);
            }
            return Color::new();
        }

        let dir = ray.direction().normalized();
        let t = 0.5 * (dir.y + 1.0);
        let v = (1.0 - t) * Vector3::xyz(1.0, 1.0, 1.0) + t * Vector3::xyz(0.5, 0.7, 1.0);
        Color::rgb(v.x, v.y, v.z)
    }
}
