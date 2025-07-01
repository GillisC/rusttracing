use image::{ImageBuffer, Rgba};
use indicatif::{ProgressBar, ProgressStyle};

// modules
mod vec3;

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
            let a = 255.0;

            let ir = 255.999 * r;
            let ig = 255.999 * g;
            let ib = 255.999 * b;

            img.put_pixel(x, y, Rgba([ir as u8, ig as u8, ib as u8, a as u8]));

            pb.inc(1);
        }
    }

    pb.finish_with_message("Done!");

    img.save("output_image.png").expect("Failed to save image");
}
