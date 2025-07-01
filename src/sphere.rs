use crate::hittable::{Hittable, HitRecord};
use crate::vec3::{Point3, Vec3};

pub struct Sphere {
    center: Point3,
    radius: f32
}

impl Sphere {
    pub fn new(center: &Point3, radius: f32) -> Self {
        Self {
            center: *center,
            radius: radius.max(0.0),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &crate::ray::Ray, ray_tmin: f32, ray_tmax: f32, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = Vec3::dot(&r.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h*h - a*c;
        
        if discriminant < 0.0 {
            return false;
        } 

        let sqrtd = discriminant.sqrt();
        let root = (h - sqrtd) / a;
        if ( root <= ray_tmin ) || ( ray_tmax <= root ){
            return false;
        }

        // the length which the ray traveled to hit sphere
        rec.t = root;
        // the point that was hit
        rec.point = r.at(rec.t);
        // creates a normalized vector pointing out from the hit point
        let outward_normal = (rec.point - self.center) / self.radius;
        // sets the normal and front_face depending on direction
        rec.set_face_normal(r, &outward_normal);

        return true;
    }

}
