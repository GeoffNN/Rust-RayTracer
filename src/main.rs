use eerdekens_bot::hittable_list::HittableList;
use eerdekens_bot::sphere::Sphere;
use eerdekens_bot::vec::Point3;

use eerdekens_bot::camera::Camera;
use eerdekens_bot::material;

fn three_balls_on_ground_scene() -> HittableList {
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
    return world;
}

fn main() {
    let image_path = "raytracing_level_release_1.ppm";

    // Camera
    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.num_samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(-2., 2., 1.);
    camera.lookat = Point3::new(0., 0., -1.);
    camera.v_up = Point3::new(0., 1., 0.);
    camera.defocus_angle = 10.;
    camera.focus_dist = 3.4;

    // World
    // TODO(geoff): Take world from a config file
    let world = three_balls_on_ground_scene();

    camera.render(&world, image_path);
}
