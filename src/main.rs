use ray_tracing::{Image, PPMFormatter, Pixel, Point, Ray, Sphere, Vector};

use std::{fs::File, path::Path};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    // Measured in number of Pixels
    let image_width: u16 = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u16;
    // Measured in the units of our coordinate system
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * image_width as f64 / image_height as f64;
    let eye_point = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    // The horizontal vector (in +ve x direction)
    let viewport_u = Vector {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    // The vertical vector (in -ve y direction)
    let viewport_v = Vector {
        x: 0.0,
        y: -viewport_height,
        z: 0.0,
    };
    // We linearly space our image's pixels into a grid within the viewport, with
    // the gap between the viewport boundary and a pixel being half the pixel spacing
    // These are hence the pixel to pixel deltas
    let pixel_delta_u = viewport_u / image_width.into();
    let pixel_delta_v = viewport_v / image_height.into();
    // We take the center of the viewport to be in the -ve z direction from the eye_point, and
    // the viewoprt itself in the (x,y) plane
    let viewport_upper_left: Point = eye_point - viewport_u / 2.0 - viewport_v / 2.0 - Vector {
        x: 0.0,
        y: 0.0,
        z: focal_length
    };
    let pixel00 = viewport_upper_left + (pixel_delta_u + pixel_delta_v) / 2.0;
    let sphere = Sphere {
        center: Point::new(0.0, 0.0, -2.0),
        radius: 0.5,
    };
    let colour_generator = |x: u16, y: u16| {
        let point: Point = pixel00 + (x as f64) * pixel_delta_u + (y as f64) * pixel_delta_v;
        let ray = Ray::from_two_points(eye_point, point);
        ray_colour(sphere, ray)
    };
    let image = Image::new(image_height, image_width, colour_generator);
    let mut ppm_formatter = PPMFormatter::new(true);
    let mut f = File::create(Path::new("tmp.ppm")).unwrap();
    image.write_to_file(&mut f, &mut ppm_formatter).unwrap();
}

fn ray_colour(sphere: Sphere, ray: Ray) -> Pixel {
    if hits_sphere(sphere, ray) {
        return Pixel::new(255, 0, 0)
    }
    let scale_factor = (ray.direction.unit().y + 1.0) / 2.0;
    let whiteout = ((1.0 - scale_factor) * 255.0) as u8;
    Pixel::new( whiteout, whiteout, 255)
}

fn hits_sphere(sphere: Sphere, ray: Ray) -> bool {
    let a = ray.direction.l2_norm_squared();
    let b = -2.0 * ray.direction.dot(sphere.center - ray.origin);
    let c = (sphere.center - ray.origin).l2_norm_squared() - sphere.radius.powi(2);
    b.powi(2) - 4.0 * a * c >= 0.0
}