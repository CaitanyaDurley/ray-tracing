use ray_tracing::{
    Point,
    Sphere,
    SurfaceSet,
    Camera,
};

use std::path::Path;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u16 = 800;
    let image_height = (image_width as f64 / aspect_ratio) as u16;
    // let image_width = 1920;
    // let image_height = 1200;
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * image_width as f64 / image_height as f64;
    let camera = Camera::new(image_width, image_height, viewport_width, viewport_height, focal_length, 7, 50);
    let mut world = SurfaceSet::new();
    world.add(Box::new(Sphere::new(
        Point::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Box::new(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
    )));
    camera.render(&world, Path::new("tmp.ppm")).unwrap();
}
