use rand::{prelude::ThreadRng, Rng};

use crate::vector::Vector3;

pub trait Random {
    fn random(rng: &mut ThreadRng) -> Self;
}

impl Random for f32 {
    fn random(rng: &mut ThreadRng) -> Self {
        rng.gen()
    }
}

pub trait RandomRange<T = Self> {
    fn random_range(rng: &mut ThreadRng, min: T, max: T) -> Self;
}

impl RandomRange for f32 {
    fn random_range(rng: &mut ThreadRng, min: Self, max: Self) -> Self {
        rng.gen_range(min, max)
    }
}

pub fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vector3 {
    loop {
        let p = Vector3::random_range(rng, -1.0, 1.0);
        if p.squared_length() <= 1.0 {
            return p;
        }
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
