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
    pub image_width: usize,
    pub num_samples_per_pixel: i32,
    pub max_depth: i32,
    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub v_up: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,

    image_height: usize,
    center: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_location: Vec3,
    // Camera orientation basis vectors
    u: Vec3,
    v: Vec3,
    w: Vec3,
    // Defocus disk horizontal and vertical radii
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
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
            lookfrom: Point3::new(0.0, 0.0, 0.0),
            lookat: Point3::new(0.0, 0.0, -1.0),
            v_up: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.,
            focus_dist: 10.,

            // These will be initialized in initialize
            center: Point3::new(0., 0., 0.),
            pixel_delta_u: Vec3::new(0., 0., 0.),
            pixel_delta_v: Vec3::new(0., 0., 0.),
            pixel00_location: Vec3::new(0., 0., 0.),

            u: Vec3::new(0., 0., 0.),
            v: Vec3::new(0., 0., 0.),
            w: Vec3::new(0., 0., 0.),

            defocus_disk_u: Vec3::default(),
            defocus_disk_v: Vec3::default(),
        };
        camera.initialize();
        return camera;
    }
}

impl Camera {
    fn initialize(&mut self) {
        // Camera

        self.image_height = ((self.image_width as f64 / self.aspect_ratio) as usize).max(1);

        let theta = utils::degrees_to_radians(self.vfov);
        let h = (0.5 * theta).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = (self.image_width as f64 / self.image_height as f64) * viewport_height;

        self.center = self.lookfrom;

        // Define camera basis vectors
        self.w = (self.lookfrom - self.lookat).normalize();
        self.u = self.v_up.cross(&self.w).normalize();
        self.v = self.w.cross(&self.u);

        let viewport_u = viewport_width * self.u;
        let viewport_v = -viewport_height * self.v;

        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);

        // Calculate the location of the upper left pixel.
        let viewport_upper_left_corner =
            self.center - self.focus_dist * self.w - viewport_u / 2.0 - viewport_v / 2.0;

        self.pixel00_location =
            viewport_upper_left_corner + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // Calculate the defocus disk radii
        let defocus_radius =
            self.focus_dist * utils::degrees_to_radians(self.defocus_angle / 2.).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn sample_from_defocus_disk(&self, rng: &mut rand::rngs::ThreadRng) -> Vec3 {
        let p = Vec3::random_in_unit_disk(rng);
        return self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v);
    }

    fn ray_color(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        ray: &Ray,
        world: &dyn Hittable,
        depth: i32,
    ) -> Color {
        if depth <= 0 {
            // return black
            return Color::new(0.0, 0.0, 0.0);
        }
        let mut hit_record: HitRecord = HitRecord::default();
        // Color using the normal vector
        if world.hit(ray, Interval::new(0.001, f64::INFINITY), &mut hit_record) {
            let mut attenuation = Color::default();
            let mut scattered_ray = Ray::default();
            if hit_record.material.scatter(
                rng,
                ray,
                &hit_record,
                &mut attenuation,
                &mut scattered_ray,
            ) {
                return attenuation * self.ray_color(rng, &scattered_ray, world, depth - 1);
            }
            // If no scatter than return black
            return Color::new(0.0, 0.0, 0.0);
        }

        // Background color
        let unit_direction = ray.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        return (1. - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
    }

    fn get_ray(&self, x: usize, y: usize, rng: &mut rand::rngs::ThreadRng) -> Ray {
        let mut pixel_center =
            self.pixel00_location + x as f64 * self.pixel_delta_u + y as f64 * self.pixel_delta_v;

        pixel_center += self.sample_pixel_from_square(rng);
        let ray_origin = if self.defocus_angle <= 0. {
            self.center
        } else {
            self.sample_from_defocus_disk(rng)
        };
        let ray_direction = pixel_center - ray_origin;
        let ray = Ray::new(ray_origin, ray_direction);
        return ray;
    }

    fn sample_pixel_from_square(&self, rng: &mut rand::rngs::ThreadRng) -> Vec3 {
        let x = rng.gen_range(-0.5..0.5);
        let y = rng.gen_range(-0.5..0.5);

        return x * self.pixel_delta_u + y * self.pixel_delta_v;
    }

    fn render_line(&self, world: &dyn Hittable, row_idx: usize, row: &mut [f64]) {
        let mut rng = rand::thread_rng();

        for x in 0..self.image_width {
            let mut color = Color::default();
            for _ in 0..self.num_samples_per_pixel {
                let ray = self.get_ray(x, row_idx, &mut rng);
                color += self.ray_color(&mut rng, &ray, world, self.max_depth);
            }
            row[3 * x..3 * (x + 1)].copy_from_slice(&color.to_array());
        }
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
        let mut flat_image = vec![0.; self.image_width * self.image_height * 3];
        let rows: Vec<(usize, &mut [f64])> = flat_image
            .chunks_mut(self.image_width * 3)
            .enumerate()
            .collect();

        rows.into_par_iter()
            .for_each(|(y, row)| self.render_line(world, y, row));

        // Write pixel colors in the correct order
        for y in 0..self.image_height {
            let row = &flat_image[y * self.image_width * 3..(y + 1) * self.image_width * 3];
            for x in 0..self.image_width {
                let color = Color::from_slice(&row[3 * x..3 * (x + 1)]);
                write_color(&mut writer, &color, self.num_samples_per_pixel).unwrap();
            }
        }

        writer.flush().unwrap();
    }
}
