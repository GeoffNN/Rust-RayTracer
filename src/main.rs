use eerdekens_bot::hittable_list::HittableList;
use eerdekens_bot::sphere::Sphere;
use eerdekens_bot::vec::Point3;

use eerdekens_bot::camera::Camera;

fn main() {
    let image_path = "raytracing_level_5.ppm";

    // Camera
    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.num_samples_per_pixel = 100;

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0., -100.5, -1.0), 100.)));

    camera.render(&world, image_path);
}
