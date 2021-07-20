use std::time::SystemTime;

use canvas::Canvas;
use color::Color;
use hit::Hittable;
use material::Scatterable;
use ray::Ray;
use scene::Scene;
use util::Random;
use vector::Vector3;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use std::sync::Arc;
use crate::tile::{split_surface, TileConfig};

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
pub mod tile;

pub struct RenderOptions {
    pub samples: u32,
    pub max_scatter: u32,
    pub tile_config: TileConfig,
}

pub struct Raytracer<'a, T: Canvas> {
    canvas: &'a mut T,
    options: RenderOptions,
}

impl<'a, T: Canvas> Raytracer<'a, T> {
    pub fn new(canvas: &'a mut T, options: RenderOptions) -> Raytracer<T> {
        Raytracer { canvas, options }
    }

    pub fn render(&mut self, scene: Arc<Scene>) {
        self.draw_scene(scene);
    }

    fn draw_scene(&mut self, scene: Arc<Scene>) {
        let width = self.canvas.width();
        let height = self.canvas.height();

        let samples = self.options.samples;
        let max_scatter = self.options.max_scatter;

        let pool = ThreadPool::new(4);

        let (tx, rx) = channel();

        let tile_width = self.options.tile_config.width;
        let tile_height = self.options.tile_config.height;
        let mut tiles = split_surface(width, height, tile_width, tile_height);
        let total_tiles = tiles.len();

        let now = SystemTime::now();
        for mut tile in tiles.drain(..) {
            let tx = tx.clone();
            let scene = scene.clone();
            pool.execute(move || {
                for j in 0..tile.height {
                    for i in 0..tile.width {
                        let mut color = Color::new();
                        for _ in 0..samples {
                            let x = i + tile.x;
                            let y = height - (j + tile.y + 1);
                            let dx = f32::random();
                            let dy = f32::random();
                            let u = (x as f32 + dx) / width as f32;
                            let v = (y as f32 + dy) / height as f32;
                            let ray = scene.camera.ray(u, v);
                            color += Raytracer::<T>::ray_color(&ray, &scene, max_scatter);
                        }

                        color.r = (color.r / samples as f32).sqrt();
                        color.g = (color.g / samples as f32).sqrt();
                        color.b = (color.b / samples as f32).sqrt();
                        let index = (i * tile.height + j) as usize;
                        tile.data[index] = color;
                    }
                }
                tx.send(tile).unwrap();
            });
        }

        drop(tx);

        let mut current_progress = 0;
        for tile in rx.iter() {
            self.canvas.draw_tile(&tile);
            current_progress += 1;
            println!("Tile rendered [{}/{}]", current_progress, total_tiles);
        }
        println!("Done: {} ms", now.elapsed().unwrap().as_millis());
    }

    fn ray_color(ray: &Ray, scene: &Scene, scatters_count: u32) -> Color {
        if scatters_count == 0 {
            return Color::new();
        }

        if let Some(hit) = scene.hit(ray, 0.001, f32::INFINITY) {
            if let Some(scatter) = hit.material.scatter(ray, &hit) {
                return scatter.attenuation * Raytracer::<T>::ray_color(&scatter.ray, scene, scatters_count - 1);
            }
            return Color::new();
        }

        let dir = ray.direction().normalized();
        let t = 0.5 * (dir.y + 1.0);
        let v = (1.0 - t) * Vector3::xyz(1.0, 1.0, 1.0) + t * Vector3::xyz(0.5, 0.7, 1.0);
        Color::rgb(v.x, v.y, v.z)
    }
}
