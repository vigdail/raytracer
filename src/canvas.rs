use sdl2::{pixels, rect::Point, video::Window};

use crate::Color;
use crate::tile::Tile;

pub trait Canvas {
    fn draw_point(&mut self, color: &Color, x: u32, y: u32);
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn clear(&mut self);
    fn flush(&mut self);
    fn draw_tile(&mut self, tile: &Tile) {
        for x in 0..tile.width {
            for y in 0..tile.height {
                let index = (x * tile.height + y) as usize;
                let color = tile.data[index];
                self.draw_point(&color, x + tile.x, y + tile.y);
            }
        }
        self.flush();
    }
}

impl Canvas for sdl2::render::Canvas<Window> {
    fn draw_point(&mut self, color: &Color, x: u32, y: u32) {
        self.set_draw_color(pixels::Color::RGBA(
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8,
            (color.a * 255.0) as u8,
        ));
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
