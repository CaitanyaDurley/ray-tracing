use crate::image::{
    Pixel,
    Image,
    formatter::ImageFormatter,
};

use std::iter;

pub struct PPMFormatter {
    ascii_mode: bool,
}

impl PPMFormatter {
    pub fn new(ascii_mode: bool) -> Self {
        Self {
            ascii_mode,
        }
    }

    fn header(&self, image: &Image) -> String {
        let magic_number = if self.ascii_mode {
            "P3"
        } else {
            "P6"
        };
        format!("{}\n{} {}\n255\n", magic_number, image.width, image.height)
    }
}

impl ImageFormatter for PPMFormatter {
    fn get_bytes(&mut self, image: Image) -> impl Iterator<Item = Vec<u8>> {
        let pixel_to_bytes = if self.ascii_mode {
            |pixel: Pixel| format!("{} {} {}\n", pixel.red, pixel.green, pixel.blue).into_bytes()
        } else {
            |pixel: Pixel| vec![pixel.red, pixel.green, pixel.blue]
        };
        Iterator::chain(
            iter::once(self.header(&image).into_bytes()),
            image.pixels.map(pixel_to_bytes),
        )
    }
    
    fn len(&self, image: &Image) -> u64 {
        // RBG for each pixel
        let mut pixel_bytes: u64 = (image.width as u64) * (image.height as u64) * 3;
        if self.ascii_mode {
            // Each R/G/B value is upto 3 chars long (255) followed by " " or "\n"
            pixel_bytes *= 4
        }
        self.header(image).len() as u64 + pixel_bytes
    }

    
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::image::Pixel;

    #[test]
    fn binary_mode_header() {
        let mut f = PPMFormatter::new(false);
        let image = Image::new(1, 2, &|_c, _r| Pixel::black());
        let header = f.get_bytes(image).next();
        assert_eq!(header, Some(b"P6\n2 1\n255\n".to_vec()));
    }

    #[test]
    fn ascii_mode_header() {
        let mut f = PPMFormatter::new(true);
        let image = Image::new(1, 2, &|_c, _r| Pixel::black());
        let header = f.get_bytes(image).next();
        assert_eq!(header, Some(b"P3\n2 1\n255\n".to_vec()));
    }

    #[test]
    fn ascii_mode_ends_with_newline() {
        let mut f = PPMFormatter::new(true);
        let image = Image::new(1, 2, &|_c, _r| Pixel::black());
        let last = f.get_bytes(image).last().unwrap();
        assert_eq!(last[last.len() - 1], b'\n');
    }
}