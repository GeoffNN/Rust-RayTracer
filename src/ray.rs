use crate::vec::{Point3, Vec3};

pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
