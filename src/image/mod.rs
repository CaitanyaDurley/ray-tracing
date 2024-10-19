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
    height: u32,
    width: u32,
    pixels: Vec<Pixel>,
}

impl Image {
    pub fn new<F>(height: u32, width: u32, colour: F) -> Self
        where F: Fn(u32, u32) -> Pixel
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
