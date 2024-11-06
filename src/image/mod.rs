pub mod formatter;

use formatter::ImageFormatter;

use std::fs::File;
use std::io::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
}

impl Pixel {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue,
        }
    }

    pub fn black() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

impl From<Pixel> for u32 {
    /// Represent the Pixel as a u32 by storing the 3
    /// bytes consecutively (with a leading zero byte)
    /// # Example
    /// ```
    /// use ray_tracing::Pixel;
    /// let crimson = Pixel::new(220, 20, 60);
    /// let actual: u32 = crimson.into();
    /// let expected = 0xdc143c;
    /// assert_eq!(actual, expected)
    /// ```
    fn from(value: Pixel) -> Self {
        [(value.red, 16), (value.green, 8), (value.blue, 0)]
            .iter()
            .map(|x| {(x.0 as u32) << x.1})
            .sum()
    }
}

/// A logical height x width grid of `Pixel`s
/// The pixels are stored internally as an iterator for memory efficiency
/// It is up to `ImageFormatter`s to deal with this as necessary (worst case
/// is just `image.pixels.collect()`)
pub struct Image<'a> {
    pub height: u16,
    pub width: u16,
    pixels: Box<dyn 'a + Iterator<Item = Pixel>>,
}


impl<'a> Image<'a> {
    /// Create an `Image` from a colour generator closure
    /// `colour`'s first argument is the column `Pixel` index (i.e. in the horizontal direction)
    /// `colour`'s second argument is the row `Pixel` index (i.e. in the vertical direction)
    /// # Example
    /// ```
    /// use ray_tracing::{Pixel, Image};
    /// let colour = |col, row| Pixel::new(0, 100 * row as u8, 50 * col as u8);
    /// let image = Image::new(2, 3, &colour);
    /// let pixels = image.collect();
    /// assert_eq!(pixels[0], Pixel::black());
    /// assert_eq!(pixels[4], Pixel::new(0, 100, 50));
    /// ```
    pub fn new<F>(height: u16, width: u16, colour: &'a F) -> Self
        where F: Fn(u16, u16) -> Pixel
    {
        let pixels = (0..height).flat_map(move |r| (0..width).map(move |c| colour(c, r)));
        Self {
            height,
            width,
            pixels: Box::new(pixels),
        }
    }

    pub fn collect(self) -> Vec<Pixel> {
        self.pixels.collect()
    }

    pub fn write_to_file<T: ImageFormatter>(self, f: &mut File, formatter: &mut T) -> io::Result<()> {
        let mut stdout = io::stdout();
        let size = formatter.len(&self) as f64;
        let mut count= 0;
        for data in formatter.get_bytes(self) {
            f.write_all(&data)?;
            count += data.len();
            if 0 == count % 1000 {
                stdout.write_all(format!("\rWritten {:.1}%", 100.0 * (count as f64) / size).as_bytes())?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn red_pixel_to_u32() {
        let red = Pixel::new(255, 0, 0);
        let res: u32 = red.into();
        assert_eq!(res, 0x00ff0000);
    }

    #[test]
    fn green_pixel_to_u32() {
        let green = Pixel::new(0, 255, 0);
        let res: u32 = green.into();
        assert_eq!(res, 0x0000ff00);
    }

    #[test]
    fn blue_pixel_to_u32() {
        let blue = Pixel::new(0, 0, 255);
        let res: u32 = blue.into();
        assert_eq!(res, 0x000000ff);
    }

    #[test]
    fn new_image_has_correct_num_pixels() {
        let image = Image::new(3, 4, &|_c, _r| Pixel::black());
        assert_eq!(image.collect().len(), 12);
    }

    #[test]
    fn very_large_image() {
        let width = u16::MAX;
        let image = Image::new(1, width, &|_c, _r| Pixel::black());
        assert_eq!(image.collect().len(), width.into());
    }
}