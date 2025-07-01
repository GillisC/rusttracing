use crate::vec3::*;

pub struct Ray {
    origin: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: &Point3, dir: &Vec3) -> Self {
        Self {
            origin: *origin,
            dir: *dir,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin() + self.direction() * t
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let origin = Vec3::new();
        let dir = Vec3::build(1.0, 0.0, 0.0);
        let r = Ray::new(&origin, &dir);
        assert_eq!(r.origin, origin);
        assert_eq!(r.dir, dir);
        assert_eq!(r.origin(), origin);
        assert_eq!(r.direction(), dir);
    }

    #[test]
    fn test_at() {
        let origin = Vec3::new();
        let dir = Vec3::build(1.0, 0.0, 0.0);
        let r = Ray::new(&origin, &dir);
        let at = r.at(2.0);
        assert_eq!(at.x, 2.0);
    }

}
