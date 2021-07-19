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
use raytracer::camera::Camera;
use raytracer::canvas::Canvas;

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

    let scene = create_scene(canvas.width(), canvas.height());
    let mut rt = Raytracer::new(&mut canvas, RenderOptions {samples: 10, max_scatter: 10});

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

fn create_scene(width: u32, height: u32) -> Scene {
    let look_from = Vector3::xyz(13.0, 2.0, 3.0);
    let look_at = Vector3::xyz(0.0, 0.0, 0.0);
    let dist_to_focus = (look_from - look_at).length();

    let camera = Camera::new(
        look_from,
        look_at,
        Vector3::xyz(0.0, 1.0, 0.0),
        60.0,
        width as f32 / height as f32,
        0.1,
        dist_to_focus,
    );

    let ground_mat = Material::Lambertian(Lambertian {
        albedo: Color::rgb(0.5, 0.5, 0.5),
    });

    let mut scene = Scene::new(camera);
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
