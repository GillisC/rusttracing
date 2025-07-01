use image::{ImageBuffer, Rgba};
use indicatif::{ProgressBar, ProgressStyle};

// modules
mod vec3;
mod color;

use crate::color::{Color, write_color};

fn main() {
    let width: u32 = 1000;
    let height: u32 = 1000;

    let total: u64 = u64::from( width * height );
    let pb = ProgressBar::new(total);

    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    let mut img = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let r = f64::from(x) / f64::from(width - 1);
            let g = f64::from(y) / f64::from(height - 1);
            let b = 0.0;
           
            let pixel_color = Color::build(r, g, b);
            write_color(x, y, &mut img, &pixel_color);
            
            pb.inc(1);
        }
    }

    pb.finish_with_message("Done!");

    img.save("output_image.png").expect("Failed to save image");
}
