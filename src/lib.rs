use canvas::Canvas;
use color::Color;
use entity::sphere::Sphere;
use hit::Hittable;
use ray::Ray;
use scene::Scene;
use vector::Vector3;

pub mod canvas;
pub mod color;
pub mod entity;
pub mod hit;
pub mod ray;
pub mod scene;
pub mod vector;

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

        let aspect = width as f32 / height as f32;

        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect;
        let focal_length = 1.0;

        let origin = Vector3::new();
        let horizontal = Vector3::xyz(viewport_width, 0.0, 0.0);
        let vertical = Vector3::xyz(0.0, viewport_height, 0.0);
        let left_botton =
            origin - horizontal * 0.5 - vertical * 0.5 - Vector3::xyz(0.0, 0.0, focal_length);

        let scene = create_scene();

        for j in 0..height {
            for i in 0..width {
                let u = i as f32 / width as f32;
                let v = j as f32 / height as f32;

                let direction = left_botton + u * horizontal + v * vertical - origin;
                let ray = Ray::new(origin, direction);
                let color = ray_color(&ray, &scene);

                self.canvas.draw_point(&color, i, height - j - 1);
            }
        }
        println!("Done");
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
    scene.add(Box::new(Sphere::new(Vector3::xyz(1.0, 0.0, -1.0), 0.5)));
    scene.add(Box::new(Sphere::new(Vector3::xyz(-1.0, 0.0, -1.0), 0.5)));

    scene
}
