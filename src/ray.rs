use crate::color::Color;
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

pub fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let ray_origin_to_center = ray.origin - center;
    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * ray_origin_to_center.dot(&ray.direction);
    let c = ray_origin_to_center.length_squared() - radius.powi(2);
    let discriminant = b.powi(2) - 4.0 * a * c;

    if discriminant < 0.0 {
        return -1.0;
    }

    return (-b - discriminant.sqrt()) / (2.0 * a);
}

pub fn ray_color(ray: &Ray) -> Color {
    let sphere_center = Point3::new(0.0, 0.0, -1.0);
    let sphere_radius = 0.5;

    let t = hit_sphere(sphere_center, sphere_radius, ray);
    if t > 0.0 {
        let normal_vector = (ray.at(t) - sphere_center).normalize();
        return 0.5
            * Vec3::new(
                normal_vector.x + 1.,
                normal_vector.y + 1.,
                normal_vector.z + 1.,
            );
    }

    let direction = ray.direction.normalize();
    let t = 0.5 * (direction.y + 1.0) as f64;
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
