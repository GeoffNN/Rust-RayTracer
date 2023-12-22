use eerdekens_bot::hittable_list::HittableList;
use eerdekens_bot::sphere::Sphere;
use eerdekens_bot::vec::Point3;

use eerdekens_bot::camera::Camera;
use eerdekens_bot::material;

fn main() {
    let image_path = "raytracing_level_10.ppm";

    // Camera
    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.num_samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.vfov = 90.0;

    // TODO(geoff): Take this from a config file

    // Materials
    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(
        Point3::new(0., 0., -1.),
        0.5,
        &material::MATERIAL_COPPER,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-0.7, 0., -1.),
        0.3,
        &material::MATERIAL_RED_PLASTIC,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1., 0., -1.),
        0.5,
        &material::MATERIAL_SILVER,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0., -100.5, -1.0),
        100.,
        &material::MATERIAL_GROUND,
    )));

    camera.render(&world, image_path);
}
