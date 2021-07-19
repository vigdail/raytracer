use std::time::Duration;

use sdl2::{event::Event, keyboard::Keycode};

use raytracer::{Raytracer, RenderOptions};
use raytracer::scene::Scene;
use raytracer::material::{Material, Lambertian, Dielectric, Metal};
use raytracer::color::Color;
use raytracer::entity::Entity;
use raytracer::entity::sphere::Sphere;
use raytracer::vector::Vector3;
use raytracer::util::Random;

const FPS: u32 = 60;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let scale = 3;

    let window = video_subsystem
        .window("Raytracer", 1920 / scale, 1080 / scale)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut rt = Raytracer::new(&mut canvas, RenderOptions {samples: 10, max_scatter: 10});
    let scene = create_scene();

    let mut event_pump = sdl_context.event_pump()?;

    rt.render(&scene);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    }
    Ok(())
}

fn create_scene() -> Scene {
    let ground_mat = Material::Lambertian(Lambertian {
        albedo: Color::rgb(0.5, 0.5, 0.5),
    });

    let mut scene = Scene::new();
    scene.add(Entity::Sphere(Sphere::new(
        Vector3::xyz(0.0, -1000.0, 0.0),
        1000.0,
        ground_mat,
    )));

    let size = 11;
    for i in -size..size {
        for j in -size..size {
            let material = Material::random();
            let center = Vector3::xyz(
                i as f32 + 0.9 * f32::random(),
                0.2,
                j as f32 + 0.9 * f32::random(),
            );

            let sphere = Sphere::new(center, 0.2, material);

            scene.add(Entity::Sphere(sphere));
        }
    }

    let material1 = Material::Dielectric(Dielectric::new(1.5));
    scene.add(Entity::Sphere(Sphere::new(
        Vector3::xyz(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Material::Lambertian(Lambertian::new(Color::rgb(0.4, 0.2, 0.1)));
    scene.add(Entity::Sphere(Sphere::new(
        Vector3::xyz(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Material::Metal(Metal::new(Color::rgb(0.4, 0.2, 0.1), 0.0));
    scene.add(Entity::Sphere(Sphere::new(
        Vector3::xyz(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    scene
}
