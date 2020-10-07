use std::time::SystemTime;

use camera::Camera;
use canvas::Canvas;
use color::Color;
use entity::{sphere::Sphere, Entity};
use hit::Hittable;
use material::{Dielectric, Lambertian, Material, Metal, Scatterable};
use ray::Ray;
use scene::Scene;
use util::Random;
use vector::Vector3;

mod camera;
mod canvas;
mod color;
mod entity;
mod hit;
mod material;
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

        let camera = Camera::new(
            Vector3::xyz(-2.0, 2.0, 1.0),
            Vector3::xyz(0.0, 0.0, -1.0),
            Vector3::xyz(0.0, 1.0, 0.0),
            90.0,
            width as f32 / height as f32,
        );

        let scene = create_scene();

        let samples = 100;
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

    fn ray_color(&mut self, ray: &Ray, scene: &Scene, depht: u32) -> Color {
        if depht <= 0 {
            return Color::new();
        }

        if let Some(hit) = scene.hit(ray, 0.001, std::f32::INFINITY) {
            if let Some(scatter) = hit.material.scatter(ray, &hit) {
                return scatter.attenuation * self.ray_color(&scatter.ray, scene, depht - 1);
            }
            return Color::new();
        }

        let dir = ray.direction().normalized();
        let t = 0.5 * (dir.y + 1.0);
        let v = (1.0 - t) * Vector3::xyz(1.0, 1.0, 1.0) + t * Vector3::xyz(0.5, 0.7, 1.0);
        Color::rgb(v.x, v.y, v.z)
    }
}

fn create_scene() -> Scene {
    let ground_mat = Material::Lambertian(Lambertian {
        albedo: Color::rgb(0.8, 0.8, 0.0),
    });
    let central_mat = Material::Lambertian(Lambertian {
        albedo: Color::rgb(0.7, 0.3, 0.3),
    });
    let left_mat = Material::Dielectric(Dielectric::new(1.5));
    let right_mat = Material::Metal(Metal::new(Color::rgb(0.8, 0.6, 0.2), 1.0));

    let mut scene = Scene::new();
    scene.add(Entity::Sphere(Sphere::new(
        Vector3::xyz(0.0, -100.5, -1.0),
        100.0,
        ground_mat,
    )));

    scene.add(Entity::Sphere(Sphere::new(
        Vector3::xyz(0.0, 0.0, -1.0),
        0.5,
        central_mat,
    )));
    scene.add(Entity::Sphere(Sphere::new(
        Vector3::xyz(-1.0, 0.0, -1.0),
        0.5,
        left_mat,
    )));
    scene.add(Entity::Sphere(Sphere::new(
        Vector3::xyz(1.0, 0.0, -1.0),
        0.5,
        right_mat,
    )));

    scene
}
