use std::f32::consts::PI;

pub const INFINITY: f32 = f32::INFINITY;


pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn random_float() -> f32 {
    rand::random::<f32>()
}

pub fn random_range(min: f32, max: f32) -> f32 {
    let range = max - min;
    min + (rand::random::<f32>() * range)
}

