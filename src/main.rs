mod write_image_file;

use eerdekens_bot::ray::{ray_color, Ray};
use eerdekens_bot::vec::{Point3, Vec3};
use std::fs::File;

use eerdekens_bot::color::{write_color, Color};
use std::io::{BufWriter, Write};

fn initial_main() {
    let image_path = "image_with_color_class.ppm";
    write_image_file::write_gradient_image(image_path);
}

fn main() {
    let image_path = "raytracing_level_0.ppm";

    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 400;

    // Compute image height and make sure it's at least 1
    let image_height: i32 = ((image_width as f64 / aspect_ratio) as i32).max(1);

    // Camera
    let focal_length = 1.;
    let viewport_height = 2.0;
    let viewport_width = (image_height as f64 / image_width as f64) * viewport_height;

    let camera_center = Point3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / (image_width as f64);
    let pixel_delta_v = viewport_v / (image_height as f64);

    // Calculate the location of the upper left pixel.
    let viewport_upper_left_corner =
        camera_center - Vec3::new(0., 0., focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

    let pixel00_location = viewport_upper_left_corner + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Write to a file
    let file = File::create(image_path).unwrap();
    let mut writer = BufWriter::new(file);

    // Write the header
    writer
        .write_fmt(format_args!("P3\n{} {}\n255\n", image_width, image_height))
        .unwrap();

    for y in 0..image_height {
        // Flush stdout to update the same line in the terminal
        for x in 0..image_width {
            let pixel_center =
                pixel00_location + x as f64 * pixel_delta_u + y as f64 * pixel_delta_v;
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);
            let color = ray_color(&ray);
            write_color(&mut writer, &color).unwrap();
        }
    }
    writer.flush();
}
