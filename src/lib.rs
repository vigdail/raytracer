use std::time::SystemTime;

use camera::Camera;
use canvas::Canvas;
use color::Color;
use entity::sphere::Sphere;
use hit::Hittable;
use ray::Ray;
use scene::Scene;
use util::Random;
use vector::Vector3;

mod camera;
mod canvas;
mod color;
mod entity;
mod hit;
mod ray;
mod scene;
mod util;
mod vector;

pub struct Raytacer<'a> {
    canvas: &'a mut dyn Canvas,
}

impl<'a> Raytacer<'a> {
    pub fn new(canvas: &'a mut dyn Canvas) -> Raytacer {
        Raytacer { canvas }
    }

    pub fn render(&mut self) {
        self.draw_scene();
    }

    fn draw_scene(&mut self) {
        let width = self.canvas.width();
        let height = self.canvas.height();

        let camera = Camera::new();

        let scene = create_scene();

        let samples = 4;
        let mut random = Random::new();

        let now = SystemTime::now();

        for j in 0..height {
            for i in 0..width {
                let mut color = Color::new();
                for _ in 0..samples {
                    let di = random.random();
                    let dj = random.random();
                    let u = (i as f32 + di) / width as f32;
                    let v = (j as f32 + dj) / height as f32;
                    let ray = camera.ray(u, v);
                    color += ray_color(&ray, &scene);
                }

                color = color / samples as f32;
                self.canvas.draw_point(&color, i, height - j - 1);
            }
        }

        println!("Done: {} ms", now.elapsed().unwrap().as_millis());
    }
}

fn ray_color(ray: &Ray, scene: &Scene) -> Color {
    if let Some(hit) = scene.hit(ray, 0.0, 1.0) {
        let c = 0.5 * (hit.normal + Vector3::xyz(1.0, 1.0, 1.0));
        return Color::rgb(c.x, c.y, c.z);
    }

    let dir = ray.direction().normalized();
    let t = 0.5 * (dir.y + 1.0);
    let v = (1.0 - t) * Vector3::xyz(1.0, 1.0, 1.0) + t * Vector3::xyz(0.5, 0.7, 1.0);
    Color::rgb(v.x, v.y, v.z)
}

fn create_scene() -> Scene {
    let mut scene = Scene::new();
    scene.add(Box::new(Sphere::new(
        Vector3::xyz(0.0, -100.5, -1.0),
        100.0,
    )));
    scene.add(Box::new(Sphere::new(Vector3::xyz(0.0, 0.0, -1.0), 0.5)));

    scene
}
