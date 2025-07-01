

use image::{ImageBuffer, Rgba};
use crate::vec3::*;

pub type Color = Vec3;

pub fn write_color(x: u32, y: u32, img_buffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, color: &Color) {
    let r = color.x;
    let g = color.y;
    let b = color.z;

    let rbyte: u8 = (255.999 * r) as u8;
    let gbyte: u8 = (255.999 * g) as u8;
    let bbyte: u8 = (255.999 * b) as u8;

    img_buffer.put_pixel(x, y, Rgba([rbyte, gbyte, bbyte, 255]));
}
