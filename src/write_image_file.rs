use std::fs::File;

use eerdekens_bot::color::{write_color, Color};
use std::io::{BufWriter, Write};

pub fn write_gradient_image(image_path: &str) {
    let image_width: i32 = 640;
    let image_height: i32 = 480;

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
            // Write a pixel
            let color = Color::new(
                x as f64 / image_width as f64,
                y as f64 / image_height as f64,
                0.0,
            );
            write_color(&mut writer, &color).unwrap();
        }
    }
    writer.flush();
}
