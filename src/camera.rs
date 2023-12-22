use crate::color::{write_color, Color};
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utils;
use crate::vec::{Point3, Vec3};
use rand::Rng;

use rayon::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub num_samples_per_pixel: i32,
    pub max_depth: i32,
    pub vfov: f64,

    image_height: i32,
    center: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_location: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        let mut camera = Self {
            aspect_ratio: 1.0,
            image_width: 100,
            image_height: 100,
            num_samples_per_pixel: 100,
            max_depth: 10,
            vfov: 90.0,
            center: Point3::new(0., 0., 0.),
            pixel_delta_u: Vec3::new(0., 0., 0.),
            pixel_delta_v: Vec3::new(0., 0., 0.),
            pixel00_location: Vec3::new(0., 0., 0.),
        };
        camera.initialize();
        return camera;
    }
}

impl Camera {
    fn initialize(&mut self) {
        // Camera

        self.image_height = ((self.image_width as f64 / self.aspect_ratio) as i32).max(1);

        let focal_length = 1.;

        let theta = utils::degrees_to_radians(self.vfov);
        let h = (0.5 * theta).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = (self.image_width as f64 / self.image_height as f64) * viewport_height;

        self.center = Point3::new(0.0, 0.0, 0.0);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);

        // Calculate the location of the upper left pixel.
        let viewport_upper_left_corner =
            self.center - Vec3::new(0., 0., focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

        self.pixel00_location =
            viewport_upper_left_corner + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn ray_color(&self, ray: &Ray, world: &dyn Hittable, depth: i32) -> Color {
        if depth <= 0 {
            // return black
            return Color::new(0.0, 0.0, 0.0);
        }
        let mut hit_record: HitRecord = HitRecord::default();
        // Color using the normal vector
        if world.hit(ray, Interval::new(0.001, f64::INFINITY), &mut hit_record) {
            let mut attenuation = Color::default();
            let mut scattered_ray = Ray::default();
            if hit_record
                .material
                .scatter(ray, &hit_record, &mut attenuation, &mut scattered_ray)
            {
                let bounce_direction = scattered_ray.direction;
                return attenuation
                    * self.ray_color(&Ray::new(hit_record.p, bounce_direction), world, depth - 1);
            }
            // If no scatter than return black
            return Color::new(0.0, 0.0, 0.0);
        }

        // Background color
        let unit_direction = ray.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        return (1. - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
    }

    fn get_ray(&self, x: i32, y: i32) -> Ray {
        let mut pixel_center =
            self.pixel00_location + x as f64 * self.pixel_delta_u + y as f64 * self.pixel_delta_v;

        pixel_center += self.sample_pixel_from_square();
        let ray_direction = pixel_center - self.center;
        let ray = Ray::new(self.center, ray_direction);
        return ray;
    }

    fn sample_pixel_from_square(&self) -> Vec3 {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-0.5..0.5);
        let y = rng.gen_range(-0.5..0.5);

        return x * self.pixel_delta_u + y * self.pixel_delta_v;
    }

    pub fn render(&mut self, world: &dyn Hittable, image_path: &str) {
        self.initialize();
        // Write to a file
        let file = File::create(image_path).unwrap();
        let mut writer = BufWriter::new(file);

        // TODO(geoff): output a better format, like png or jpg
        // Write the header
        writer
            .write_fmt(format_args!(
                "P3\n{} {}\n255\n",
                self.image_width, self.image_height
            ))
            .unwrap();

        // Collect pixel colors in parallel
        let pixel_colors: Vec<Vec<Color>> = (0..self.image_height)
            .into_par_iter()
            .map(|y| {
                let mut row_colors = Vec::new();
                for x in 0..self.image_width {
                    let mut color = Color::default();
                    for _ in 0..self.num_samples_per_pixel {
                        let ray = self.get_ray(x, y);
                        color += self.ray_color(&ray, world, self.max_depth);
                    }
                    row_colors.push(color);
                }
                row_colors
            })
            .collect();
        // Write pixel colors in the correct order
        for row in pixel_colors {
            for color in row {
                write_color(&mut writer, &color, self.num_samples_per_pixel).unwrap();
            }
        }

        writer.flush().unwrap();
    }
}
