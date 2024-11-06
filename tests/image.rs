use std::io::{Read, Seek, SeekFrom};

use ray_tracing::{Pixel, Image, ImageFormatter};

#[test]
fn u32_leading_zeros() {
    for red in 0..=255 {
        for green in 0..=255 {
            for blue in 0..=255 {
                let p = Pixel::new(red, green, blue);
                assert_eq!(u32::from(p) >> 24, 0);
            }
        }
    }
}

struct DummyFormatter;

impl ImageFormatter for DummyFormatter {
    fn get_bytes(&mut self, image: Image) -> impl Iterator<Item = Vec<u8>> {
        let height = image.height as u8;
        let width = image.width as u8;
        (0..height).flat_map(move |r| (0..width).map(move |c| vec![r, c]))
    }
    
    fn len(&self, image: &Image) -> u64 {
        2 * (image.width as u64) * (image.height as u64)
    }
}

#[test]
fn writes_to_file_faithfully() {
    let image = Image::new(2, 3, &|_c, _r| Pixel::black());
    let image2 = Image::new(2, 3, &|_c, _r| Pixel::black());
    let mut dummy_formatter = DummyFormatter {};
    let mut tmpfile = tempfile::tempfile().unwrap();
    image.write_to_file(&mut tmpfile, &mut dummy_formatter).unwrap();
    let mut actual = Vec::new();
    tmpfile.seek(SeekFrom::Start(0)).unwrap();
    tmpfile.read_to_end(&mut actual).unwrap();
    let expected: Vec<u8> = dummy_formatter.get_bytes(image2).flatten().collect();
    assert_eq!(actual, expected);
}
