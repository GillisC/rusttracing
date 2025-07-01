use image::{ImageBuffer, };
use indicatif::{ProgressBar, ProgressStyle};

// modules
mod vec3;
mod color;
mod ray;

use crate::color::{Color, write_color};
use crate::vec3::*;
use crate::ray::*;

fn ray_color(r: &Ray) -> Color {
    let unit_direction = Vec3::normalize(&r.direction());
    let a = 0.5*(unit_direction.y() + 1.0);
    Color::build(1.0, 1.0, 1.0) * (1.0 - a) + Color::build(0.5, 0.7, 1.0) * a
}


fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height = (image_width as f32 / aspect_ratio) as u32;

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width / image_height) as f32;
    let camera_center = Point3::new();

    let viewport_u = Vec3::build(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::build(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    let viewport_upper_left = camera_center - Vec3::build(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel_start_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    let total: u64 = u64::from( image_width * image_height );
    let pb = ProgressBar::new(total);

    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    let mut img = ImageBuffer::new(image_width, image_height);

    for y in 0..image_height {
        for x in 0..image_width {
            let pixel_center = pixel_start_loc + (pixel_delta_u * y as f32) + (pixel_delta_v * y as f32);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(&camera_center, &ray_direction);
           
            let pixel_color = ray_color(&ray);
            write_color(x, y, &mut img, &pixel_color);
            
            pb.inc(1);
        }
    }

    pb.finish_with_message("Done!");

    img.save("output_image.png").expect("Failed to save image");
}
