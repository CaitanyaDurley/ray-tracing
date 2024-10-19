use crate::image::*;

use std::iter;

pub struct PPMFormatter {}

impl ImageFormatter for PPMFormatter {
    fn get_bytes(&mut self, image: &Image) -> impl Iterator<Item = Vec<u8>> {
        Iterator::chain(
            iter::once(format!("P3\n{} {}\n255\n", image.width, image.height).into_bytes()),
            PPMIterator::new(image.height, image.width, &image.pixels)
        )
    }
}

struct PPMIterator<'a> {
    rows: u32,
    cols: u32,
    pixels: &'a[Pixel],
    ix: u64,
}

impl<'a> PPMIterator<'a> {
    fn new(rows: u32, cols: u32, pixels: &'a[Pixel]) -> Self {
        Self {
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
        return Some(format!("{} {} {}\n", pixel.red, pixel.green, pixel.blue).into_bytes())
    }
}
