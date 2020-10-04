use canvas::Canvas;

pub mod canvas;
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
}

pub struct Raytacer<'a> {
    background: Color,
    canvas: &'a mut dyn Canvas,
}

impl<'a> Raytacer<'a> {
    pub fn new(canvas: &'a mut dyn Canvas) -> Raytacer {
        Raytacer {
            background: Color::new(),
            canvas,
        }
    }

    pub fn render(&mut self) {
        self.canvas.set_color(&self.background);
        self.canvas.clear();

        self.draw_gradient();
    }

    fn draw_gradient(&mut self) {
        let width = self.canvas.width();
        let height = self.canvas.height();

        let mut color = Color::new();

        color.b = 0.2;

        for j in 0..height {
            for i in 0..width {
                color.r = i as f32 / width as f32;
                color.g = (height - 1 - j) as f32 / height as f32;
                self.canvas.set_color(&color);
                self.canvas.draw_point(i, j);
            }
        }
    }
}
