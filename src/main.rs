use ray_tracing::{Image, Pixel, PPMFormatter};

use std::{fs::File, path::Path};

fn main() {
    let side = 1024;
    let gradient = |r, c| Pixel::new((255 * c / side) as u8, (255 * r / side) as u8, 0);
    let blue = Image::new(side, side, gradient);
    let mut ppm_formatter = PPMFormatter::new(true);
    let mut f = File::create(Path::new("tmp.ppm")).unwrap();
    blue.write_to_file(&mut f, &mut ppm_formatter).unwrap();
}
