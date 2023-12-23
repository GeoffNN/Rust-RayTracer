use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::Point3;

use rand::Rng;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    material: &'static dyn Material,
}

impl Sphere {
    // TODO: Check that radius is > 0
    pub fn new(center: Point3, radius: f64, material: &'static dyn Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    pub fn random(rng: &mut rand::rngs::ThreadRng) -> Self {
        let material = &*material::random_material(rng);
        let center = Point3::random(rng);
        let radius = rng.gen_range(0.0..1.0);

        Sphere {
            center: (center),
            radius: (radius),
            material: (material),
        }
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
        let discriminant_sqrt = discriminant.sqrt();
        // Check first root
        let mut t = (-half_b - discriminant_sqrt) / a;
        if !ray_t.surrounds(t) {
            // Check second root
            t = (-half_b + discriminant_sqrt) / a;
            if !ray_t.surrounds(t) {
                return false;
            }
        }
        record.t = t;
        record.p = ray.at(t);
        record.material = self.material;
        let outward_normal = (record.p - self.center) / self.radius;
        record.set_face_normal(ray, outward_normal);
        return true;
    }
}
