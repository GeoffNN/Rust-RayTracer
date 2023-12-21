use crate::vec::Vec3;

use std::io::{BufWriter, Write};

pub type Color = Vec3;

pub fn write_color<W: Write>(
    mut writer: &mut BufWriter<W>,
    pixel: &Color,
) -> Result<(), std::io::Error> {
    let r = (255.999 * pixel.x).round() as u8;
    let g = (255.999 * pixel.y).round() as u8;
    let b = (255.999 * pixel.z).round() as u8;

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
        write_color(&mut writer, &pixel).unwrap();
    }
}
