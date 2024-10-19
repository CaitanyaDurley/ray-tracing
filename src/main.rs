use ray_tracing::{Image, Pixel};
use ray_tracing::image_formatters::PPMFormatter;

use std::path::Path;

fn main() {
    let side = 1024;
    let gradient = |r, c| Pixel::new((255 * c / side) as u8, (255 * r / side) as u8, 0);
    let blue = Image::new(side, side, gradient);
    let mut ppm_formatter = PPMFormatter {};
    blue.write_to_file(Path::new("tmp.ppm"), &mut ppm_formatter).unwrap();
}
