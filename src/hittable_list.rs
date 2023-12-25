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
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.upper;

        let mut record = None;
        for object in &self.objects {
            // A hit updates temp_record
            if let Some(temp_record) = object.hit(ray, Interval::new(ray_t.lower, closest_so_far)) {
                closest_so_far = temp_record.t;
                record = Some(temp_record);
            }
        }
        return record;
    }
}
