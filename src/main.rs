mod write_image_file;
use eerdekens_bot::color::Color;
use eerdekens_bot::ray::Ray;
use eerdekens_bot::vecs::Point3;

fn main() {
    let image_path = "image_with_color_class.ppm";
    write_image_file::write_gradient_image(image_path);
}
