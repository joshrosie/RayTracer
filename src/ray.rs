use crate::vec3::{Point3, Vec3};

// consider a ray P(t) = A + t*b* where P(t) is a 3D
// point on a ray at time t. A is the ray origin and b is the ray direction.

#[derive(Default)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 { // P(t) = A + t*b
        self.orig + self.dir * t
    }
}