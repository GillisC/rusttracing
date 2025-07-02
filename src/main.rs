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

use crate::vec3::*;
use crate::hittable_list::{HittableList};
use crate::sphere::Sphere;
use crate::camera::{Camera};

fn main() {
    let camera = Camera::new(16.0 / 9.0, 800);

    let mut world = HittableList::new();
    let sphere1 = Sphere::new(&Point3::build(0.0, 0.0, -1.0), 0.5);
    let sphere2 = Sphere::new(&Point3::build(0.0, -100.5, -1.0), 100.0);

    world.add(Rc::new(sphere1));
    world.add(Rc::new(sphere2));

    camera.render(&world);
}

