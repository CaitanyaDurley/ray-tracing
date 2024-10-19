pub mod formatter;

use formatter::ImageFormatter;

use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone)]
pub struct Image {
    height: u16,
    width: u16,
    pixels: Vec<Pixel>,
}

impl Image {
    pub fn new<F>(height: u16, width: u16, colour: F) -> Self
        where F: Fn(u16, u16) -> Pixel
    {
        let size = (width as usize) * (height as usize);
        let mut pixels = Vec::with_capacity(size);
        for r in 0..height {
            for c in 0..width {
                pixels.push(colour(r, c))
            }
        }
        Self {
            height,
            width,
            pixels,
        }
    }

    pub fn write_to_file<T: ImageFormatter>(&self, filename: &Path, formatter: &mut T) -> io::Result<()> {
        let mut f = fs::File::create(filename)?;
        for data in formatter.get_bytes(&self) {
            f.write_all(&data)?;
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
        let image = Image::new(3, 4, |_r, _c| Pixel::black());
        assert_eq!(image.pixels.len(), 12);
    }

    #[test]
    fn very_large_image() {
        let width = u16::MAX;
        let image = Image::new(1, width, |_r, _c| Pixel::black());
        assert_eq!(image.pixels.len(), width.into());
    }
}