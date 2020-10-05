use rand::Rng;

pub trait Random {
    fn random() -> Self;
}

impl Random for f32 {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        rng.gen()
    }
}

pub trait RandomRange<T = Self> {
    fn random_range(min: T, max: T) -> Self;
}

impl RandomRange for f32 {
    fn random_range(min: Self, max: Self) -> Self {
        let mut rng = rand::thread_rng();
        rng.gen_range(min, max)
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
