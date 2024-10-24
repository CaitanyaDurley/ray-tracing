use crate::image::{
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
    fn get_bytes(&mut self, image: &Image) -> impl Iterator<Item = Vec<u8>> {
        Iterator::chain(
            iter::once(self.header(image).into_bytes()),
            PPMIterator::new(self.ascii_mode, image)
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

struct PPMIterator<'a> {
    ascii_mode: bool,
    image: &'a Image,
    ix: u32,
}

impl<'a> PPMIterator<'a> {
    fn new(ascii_mode: bool, image: &'a Image) -> Self {
        Self {
            ascii_mode,
            image,
            ix: 0,
        }
    }
}

impl<'a> Iterator for PPMIterator<'a> {
    type Item = Vec<u8>;
    
    fn next(&mut self) -> Option<Self::Item> {
        // return None
        if self.ix == (self.image.height as u32) * (self.image.width as u32) {
            return None
        }
        let pixel = self.image.pixels[self.ix as usize];
        self.ix += 1;
        let mut out = vec![pixel.red, pixel.green, pixel.blue];
        if self.ascii_mode {
            out = out.iter().map(ToString::to_string)
                .chain(iter::once("\n".to_string()))
                .collect::<Vec<String>>()
                .join(" ")
                .into_bytes();
        }
        return Some(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::image::Pixel;

    #[test]
    fn binary_mode_header() {
        let mut f = PPMFormatter::new(false);
        let image = Image::new(1, 2, |_r, _c| Pixel::black());
        let header = f.get_bytes(&image).next();
        assert_eq!(header, Some(b"P6\n2 1\n255\n".to_vec()));
    }

    #[test]
    fn ascii_mode_header() {
        let mut f = PPMFormatter::new(true);
        let image = Image::new(1, 2, |_r, _c| Pixel::black());
        let header = f.get_bytes(&image).next();
        assert_eq!(header, Some(b"P3\n2 1\n255\n".to_vec()));
    }

    #[test]
    fn ascii_mode_ends_with_newline() {
        let mut f = PPMFormatter::new(true);
        let image = Image::new(1, 2, |_r, _c| Pixel::black());
        let last = f.get_bytes(&image).last().unwrap();
        assert_eq!(last[last.len() - 1], b'\n');
    }
}