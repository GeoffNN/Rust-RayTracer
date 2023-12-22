use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

use std::vec::Vec;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool {
        let mut temp_record: HitRecord = *record;
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.upper;

        for object in &self.objects {
            // hit updates temp_record
            if object.hit(
                ray,
                Interval::new(ray_t.lower, closest_so_far),
                &mut temp_record,
            ) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                *record = temp_record;
            }
        }
        return hit_anything;
    }
}
