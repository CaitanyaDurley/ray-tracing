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

struct Image {
    height: u32,
    width: u32,
    pixels: Vec<Pixel>,
}