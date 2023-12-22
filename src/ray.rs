use crate::color::Color;
use crate::sphere::Sphere;
use crate::vec::{Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}

pub fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let ray_origin_to_center = ray.origin - center;
    let a = ray.direction.length_squared();
    let half_b = ray_origin_to_center.dot(&ray.direction);
    let c = ray_origin_to_center.length_squared() - radius.powi(2);
    let discriminant = half_b.powi(2) - a * c;

    if discriminant < 0.0 {
        return -1.0;
    }

    return (-half_b - discriminant.sqrt()) / a;
}
