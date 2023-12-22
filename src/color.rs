use crate::interval::Interval;
use crate::vec::Vec3;
use std::io::{BufWriter, Write};

pub type Color = Vec3;

const INTENSITY_INTERVAL: Interval = Interval {
    lower: 0.0,
    upper: 256.0,
};

pub fn write_color<W: Write>(
    mut writer: &mut BufWriter<W>,
    pixel: &Color,
    n_samples_per_pixel: i32,
) -> Result<(), std::io::Error> {
    let scale = 1.0 / n_samples_per_pixel as f64;

    let r = INTENSITY_INTERVAL.clamp(256.0 * pixel.x * scale).round() as u8;
    let g = INTENSITY_INTERVAL.clamp(256.0 * pixel.y * scale).round() as u8;
    let b = INTENSITY_INTERVAL.clamp(256.0 * pixel.z * scale).round() as u8;

    writeln!(&mut writer, "{} {} {}", r, g, b)
}

mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_write_color() {
        let pixel = Color::new(255.0, 255.0, 255.0);
        let file = File::create("test_file.ppm").unwrap();
        let mut writer = BufWriter::new(file);
        write_color(&mut writer, &pixel, 1).unwrap();
    }
}
