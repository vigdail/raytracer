use canvas::Canvas;
use ray::Ray;
use vector::Vector3;

pub mod canvas;
pub mod ray;
pub mod vector;

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }

    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r, g, b, a }
    }

    pub fn rgb(r: f32, g: f32, b: f32) -> Color {
        Color::rgba(r, g, b, 1.0)
    }
}

pub struct Raytacer<'a> {
    canvas: &'a mut dyn Canvas,
}

impl<'a> Raytacer<'a> {
    pub fn new(canvas: &'a mut dyn Canvas) -> Raytacer {
        Raytacer { canvas }
    }

    pub fn render(&mut self) {
        self.draw_gradient();
    }

    fn draw_gradient(&mut self) {
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

        for j in (0..height).rev() {
            for i in 0..width {
                let u = i as f32 / width as f32;
                let v = j as f32 / height as f32;

                let direction = left_botton + u * horizontal + v * vertical;
                let ray = Ray::new(origin, direction);
                let color = ray_color(&ray);

                self.canvas.draw_point(&color, i, height - j - 1);
            }
        }
        println!("Done");
    }
}

fn ray_color(ray: &Ray) -> Color {
    let n = ray.direction().normalized();
    let t = 0.5 * (n.y + 1.0);
    let v = (1.0 - t) * Vector3::xyz(1.0, 1.0, 1.0) + t * Vector3::xyz(0.5, 0.7, 1.0);
    Color::rgb(v.x, v.y, v.z)
}
