use crate::interval::Interval;
use crate::vec::Vec3;
use std::io::{BufWriter, Write};

pub type Color = Vec3;

impl Color {
    pub const fn new_const(r: f64, g: f64, b: f64) -> Color {
        Self { x: r, y: g, z: b }
    }

    pub fn to_array(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }

    pub fn from_slice(slice: &[f64]) -> Color {
        Color {
            x: slice[0],
            y: slice[1],
            z: slice[2],
        }
    }

    pub const fn black() -> Color {
        Self::new_const(0.0, 0.0, 0.0)
    }

    pub const fn white() -> Color {
        Self::new_const(1.0, 1.0, 1.)
    }
}

const INTENSITY_INTERVAL: Interval = Interval {
    lower: 0.0,
    upper: 256.0,
};

pub fn linear_to_gamma(x: f64) -> f64 {
    x.sqrt()
}

pub fn write_color<W: Write>(
    mut writer: &mut BufWriter<W>,
    pixel: &Color,
    n_samples_per_pixel: i32,
) -> Result<(), std::io::Error> {
    let scale = 1.0 / n_samples_per_pixel as f64;

    let r = linear_to_gamma(pixel.x() * scale);
    let g = linear_to_gamma(pixel.y() * scale);
    let b = linear_to_gamma(pixel.z() * scale);

    let r = INTENSITY_INTERVAL.clamp(256.0 * r).round() as u8;
    let g = INTENSITY_INTERVAL.clamp(256.0 * g).round() as u8;
    let b = INTENSITY_INTERVAL.clamp(256.0 * b).round() as u8;

    writeln!(&mut writer, "{} {} {}", r, g, b)
}

#[cfg(test)]
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
