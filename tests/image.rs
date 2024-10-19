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
    fn get_bytes(&mut self, _image: &Image) -> impl Iterator<Item = Vec<u8>> {
        (0..=2u8).map(|x| (0..=x).collect())
    }
}

#[test]
fn writes_to_file_faithfully() {
    let image = Image::new(3, 2, |_r, _c| Pixel::black());
    let mut dummy_formatter = DummyFormatter {};
    let mut tmpfile = tempfile::tempfile().unwrap();
    image.write_to_file(&mut tmpfile, &mut dummy_formatter).unwrap();
    let mut buffer = Vec::with_capacity(6);
    tmpfile.seek(SeekFrom::Start(0)).unwrap();
    tmpfile.read_to_end(&mut buffer).unwrap();
    assert_eq!(buffer, vec![0, 0, 1, 0, 1, 2]);
}
