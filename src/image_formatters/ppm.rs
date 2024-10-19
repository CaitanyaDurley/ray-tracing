use crate::image::*;

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
}

impl ImageFormatter for PPMFormatter {
    fn get_bytes(&mut self, image: &Image) -> impl Iterator<Item = Vec<u8>> {
        let magic_number = if self.ascii_mode {
            "P3"
        } else {
            "P6"
        };
        Iterator::chain(
            iter::once(format!("{}\n{} {}\n255\n", magic_number, image.width, image.height).into_bytes()),
            PPMIterator::new(self.ascii_mode, image.height, image.width, &image.pixels)
        )
    }
}

struct PPMIterator<'a> {
    ascii_mode: bool,
    rows: u32,
    cols: u32,
    pixels: &'a[Pixel],
    ix: u64,
}

impl<'a> PPMIterator<'a> {
    fn new(ascii_mode: bool, rows: u32, cols: u32, pixels: &'a[Pixel]) -> Self {
        Self {
            ascii_mode,
            rows,
            cols,
            pixels,
            ix: 0,
        }
    }
}

impl<'a> Iterator for PPMIterator<'a> {
    type Item = Vec<u8>;
    
    fn next(&mut self) -> Option<Self::Item> {
        // return None
        if self.ix == (self.rows as u64) * (self.cols as u64) {
            return None
        }
        let pixel = self.pixels[self.ix as usize];
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
