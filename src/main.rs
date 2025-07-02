use std::rc::Rc;

// modules
mod vec3;
mod color;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod interval;
mod camera;
mod material;

use crate::vec3::*;
use crate::hittable_list::{HittableList};
use crate::sphere::Sphere;
use crate::camera::{Camera};
use crate::material::{Material, Lambertian, Metal};

fn main() {
    let camera = Camera::new(16.0 / 9.0, 500);

    let mut world = HittableList::new();

    let ground_material = Material::Lambertian(Lambertian::new(Vec3::build(0.8, 0.8, 0.8)));
    let material_center = Material::Lambertian(Lambertian::new(Vec3::build(0.1, 0.2, 0.5)));
    let material_left = Material::Metal(Metal::new(Vec3::build(0.8, 0.8, 0.8)));
    let material_right = Material::Metal(Metal::new(Vec3::build(0.8, 0.6, 0.2)));

    let sphere1 = Sphere::new(&Point3::build(0.0, -100.5, -1.0), 100.0, ground_material);
    let sphere2 = Sphere::new(&Point3::build(0.0, 0.0, -1.2), 0.5, material_center);
    let sphere3 = Sphere::new(&Point3::build(-1.0, 0.0, -1.0), 0.5, material_left);
    let sphere4 = Sphere::new(&Point3::build(1.0, 0.0, -1.0), 0.5, material_right);

    world.add(Rc::new(sphere1));
    world.add(Rc::new(sphere2));
    world.add(Rc::new(sphere3));
    world.add(Rc::new(sphere4));

    camera.render(&world);
}

