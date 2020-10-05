use rand::{prelude::ThreadRng, Rng};

pub struct Random {
    rng: ThreadRng,
}

impl Random {
    pub fn new() -> Random {
        Random {
            rng: rand::thread_rng(),
        }
    }
    pub fn random(&mut self) -> f32 {
        self.rng.gen()
    }

    pub fn random_range(&mut self, min: f32, max: f32) -> f32 {
        self.rng.gen_range(min, max)
    }
}

pub fn deg_to_rad(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    match x {
        x if x < min => min,
        x if x > max => max,
        _ => x,
    }
}
