use crate::ray::{Ray};
use crate::hittable::{HitRecord};
use crate::color::Color;
use crate::vec3::{Vec3};

/// Material structure defines different types of materials that can be applied to hittable
/// objects.
#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}

/// The Material trait provides a method to scatter rays based on the material properties.
impl Material {
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        match self {
            Material::Lambertian(mat) => {
                mat.scatter(r_in, rec, attenuation, scattered)
            }
            Material::Metal(mat) => {
                mat.scatter(r_in, rec, attenuation, scattered)
            }
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Material::Lambertian(Lambertian::new(Color::new()))
    }
}

/// Lambertian material represents a diffuse surface that scatters light uniformly in all
/// directions.
#[derive(Copy, Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        // Create a scattered ray in a random direction around the normal
        let mut scatter_direction: Vec3 = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            // If the scatter direction is near zero, use the normal as the direction
            // to avoid degenerate cases.
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(&rec.point, &scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo: Color,
}

/// The Metal material represents a reflective surface
/// that reflects light based on the angle of incidence.
impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = r_in.direction().reflect(&rec.normal);
        *scattered = Ray::new(&rec.point, &reflected);
        *attenuation = self.albedo;
        true
    }
}


