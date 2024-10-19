mod ppm;
pub use ppm::PPMFormatter;

use crate::image::Image;

pub trait ImageFormatter {
    fn get_bytes(&mut self, image: &Image) -> impl Iterator<Item = Vec<u8>>;
}