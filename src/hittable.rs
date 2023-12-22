use crate::color::Color;
use crate::interval::Interval;
use crate::material::{Material, MATERIAL_CONCRETE};
use crate::ray::Ray;
use crate::vec::{Point3, Vec3};

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub color: Color,
    pub front_face: bool,
    pub material: &'static dyn Material,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        // outward normal is supposed to be a unit vector
        self.front_face = r.direction.dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            p: Point3::default(),
            normal: Vec3::default(),
            t: 0.0,
            color: Color::default(),
            front_face: true,
            material: &MATERIAL_CONCRETE,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool;
}
