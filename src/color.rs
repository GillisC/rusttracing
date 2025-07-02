

use image::{ImageBuffer, Rgba};
use crate::vec3::*;
use crate::interval::Interval;

pub type Color = Vec3;


pub fn linear_to_gamma(linear_component: f32) -> f32 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    0.0
}

pub fn write_color(x: u32, y: u32, img_buffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, color: &Color) {
    let mut r = color.x;
    let mut g = color.y;
    let mut b = color.z;

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);
    
    let intensity: Interval = Interval::with_bounds(0.0, 0.999);
    let rbyte: u8 = (256.0 * intensity.clamp(r)) as u8;
    let gbyte: u8 = (256.0 * intensity.clamp(g)) as u8;
    let bbyte: u8 = (256.0 * intensity.clamp(b)) as u8;

    img_buffer.put_pixel(x, y, Rgba([rbyte, gbyte, bbyte, 255]));
}
