use canvas::Canvas;

mod canvas;

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

        let mut color = Color::new();
        color.r = 0.5;
        color.g = 0.3;
        color.b = 0.2;

        self.canvas.set_color(&color);
        for i in 20..self.canvas.width() - 20 {
            self.canvas.draw_point(i, 20);
        }

        self.canvas.flush();
    }
}
