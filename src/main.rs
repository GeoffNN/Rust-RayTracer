use eerdekens_bot::hittable_list::HittableList;
use eerdekens_bot::sphere::Sphere;
use eerdekens_bot::vec::Point3;

use eerdekens_bot::camera::Camera;
use eerdekens_bot::material;
use rand::Rng;

fn add_three_balls_on_ground_scene(world: &mut HittableList) {
    world.add(Box::new(Sphere::new(
        Point3::new(0., 1., 0.),
        1.,
        &material::MATERIAL_COPPER,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-4., 1., 0.),
        1.,
        &material::MATERIAL_RED_PLASTIC,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(4., 1., 0.),
        1.,
        &material::MATERIAL_SILVER,
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        &material::MATERIAL_CONCRETE,
    )));
}

fn add_random_balls(world: &mut HittableList, n_balls: usize) {
    let mut rng = rand::thread_rng();
    for _ in 0..n_balls {
        let a = rng.gen_range(-11..11);
        let b = rng.gen_range(-11..11);
        let material = material::random_material(&mut rng);
        let center = Point3::new(
            a as f64 + 0.9 * rng.gen::<f64>(),
            0.2,
            b as f64 + 0.9 * rng.gen::<f64>(),
        );
        let radius = 0.2;
        if (center - Point3::new(4., 0.2, 0.)).length() > 0.9 {
            world.add(Box::new(Sphere::new(center, radius, material)));
        }
    }
}

fn main() {
    let image_path = "raytracing_level_release_3.ppm";

    // World
    // TODO(geoff): Take world from a config file
    let mut world = HittableList::new();
    add_three_balls_on_ground_scene(&mut world);
    match 1 {
        0 => (),
        1 => add_random_balls(&mut world, 100),
        _ => todo!("Haven't implemented this variant yet!"),
    };

    // Camera
    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1200;

    camera.num_samples_per_pixel = 50;
    camera.max_depth = 50;

    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(13., 2., 3.);
    camera.lookat = Point3::new(0., 0., 0.);
    camera.v_up = Point3::new(0., 1., 0.);

    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.;

    camera.render(&world, image_path);
}
