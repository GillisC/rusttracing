use crate::vec3::{Point3, Vec3};
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::color::{Color, write_color};
use crate::ray::Ray;
use crate::interval::Interval;
use raytracing::{random_float};

use image::{ImageBuffer};
use indicatif::{ProgressBar, ProgressStyle};

pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: u32,
    pub samples_per_pixel: u32,

    image_height: u32,
    pixel_samples_scale: f32,
    origin: Point3,
    pixel_00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: u32) -> Self {
        let image_height = (image_width as f32 / aspect_ratio) as u32;
        let origin = Point3::new();
        let samples_per_pixel = 10;
        let focal_length = 1.0;
        let viewport_height = 2.0;

        let pixel_samples_scale = 1.0 / samples_per_pixel as f32;
        let viewport_width = viewport_height * aspect_ratio;

        let viewport_u = Vec3::build(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::build(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        let viewport_upper_left = origin 
            - Vec3::build(0.0, 0.0, focal_length) 
            - viewport_u / 2.0 
            - viewport_v / 2.0; 

        let pixel_00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel: 10,
            image_height,
            pixel_samples_scale, 
            origin,
            pixel_00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }


    pub fn render(&self, world: &HittableList) {
        let total: u64 = u64::from( self.image_width * self.image_height );
        let pb = ProgressBar::new(total);

        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} ({eta})")
            .unwrap()
            .progress_chars("#>-"));

        let mut img = ImageBuffer::new(self.image_width, self.image_height);

        for y in 0..self.image_height {
            for x in 0..self.image_width {
                let mut pixel_color = Color::new();
                // For each pixel, we will sample multiple rays 
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x, y);
                    pixel_color += self.ray_color(&ray, &world);
                }

                write_color(x, y, &mut img, &( pixel_color * self.pixel_samples_scale ));
                pb.inc(1);
            }
        }
        pb.finish_with_message("Done!");
        img.save("output_image.png").expect("Failed to save image");
    } 


    fn ray_color(&self, ray: &Ray, world: &HittableList) -> Color {
        let mut rec = HitRecord::new();
        if world.hit(&ray, &Interval::with_bounds(0.0, f32::INFINITY), &mut rec) {
            return (rec.normal + Color::build(1.0, 1.0, 1.0)) * 0.5;
        }

        let unit_direction = Vec3::normalize(&ray.direction());
        let a = 0.5*(unit_direction.y() + 1.0);
        Color::build(1.0, 1.0, 1.0) * (1.0 - a) + Color::build(0.5, 0.7, 1.0) * a
    }

    // Returns a ray from the camera to a randomly sampled point
    // around pixel at (i, j) 
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel_00_loc + 
            ( self.pixel_delta_u * (i as f32 + offset.x) ) +
            ( self.pixel_delta_v * (j as f32 + offset.y) );

        let ray_origin = self.origin;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(&ray_origin, &ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::build(random_float() - 0.5, random_float() - 0.5, 0.0)
    }
}
