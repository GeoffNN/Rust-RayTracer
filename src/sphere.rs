use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec::Point3;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    // TODO: Check that radius is > 0
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool {
        let ray_origin_to_center = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = ray_origin_to_center.dot(&ray.direction);
        let c = ray_origin_to_center.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0.0 {
            return false;
        }
        // Check first root
        let t = (-half_b - discriminant.sqrt()) / a;
        if !ray_t.surrounds(t) {
            // Check second root
            let t = (-half_b + discriminant.sqrt()) / a;
            if !ray_t.surrounds(t) {
                return false;
            }
        }
        record.t = t;
        record.p = ray.at(t);
        let outward_normal = (record.p - self.center) / self.radius;
        record.set_face_normal(ray, outward_normal);
        return true;
    }
}
