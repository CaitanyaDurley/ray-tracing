use ray_tracing::Pixel;

#[test]
fn u32_leading_zeros() {
    let white = Pixel::new(255, 255, 255);
    let b: u32 = white.into();
    assert_eq!(b >> 24, 0);
}