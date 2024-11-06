pub mod ppm;

use crate::image::Image;

pub trait ImageFormatter {
    fn get_bytes(&mut self, image: Image) -> impl Iterator<Item = Vec<u8>>;

    fn len(&self, image: &Image) -> u64;
}
