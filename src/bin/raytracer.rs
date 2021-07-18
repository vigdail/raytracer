use std::time::Duration;

use sdl2::{event::Event, keyboard::Keycode};

use raytracer::Raytracer;

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

    let mut rt = Raytracer::new(&mut canvas);

    let mut event_pump = sdl_context.event_pump()?;

    rt.render();

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

