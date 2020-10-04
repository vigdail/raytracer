use sdl2::{pixels, rect::Point, video::Window};

use crate::Color;

pub trait Canvas {
    fn set_color(&mut self, color: &Color);
    fn draw_point(&mut self, x: u32, y: u32);
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn clear(&mut self);
    fn flush(&mut self);
}

impl Canvas for sdl2::render::Canvas<Window> {
    fn set_color(&mut self, color: &Color) {
        self.set_draw_color(pixels::Color::RGBA(
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8,
            (color.a * 255.0) as u8,
        ));
    }

    fn draw_point(&mut self, x: u32, y: u32) {
        self.draw_point(Point::new(x as i32, y as i32)).unwrap();
    }

    fn width(&self) -> u32 {
        self.output_size().unwrap().0
    }

    fn height(&self) -> u32 {
        self.output_size().unwrap().1
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn flush(&mut self) {
        self.present();
    }
}
