use crate::{
    image::{
        Image,
        formatter::ppm::PPMFormatter,
    },
    geometry::{
        Point,
        Vector,
        Ray,
        Interval,
        IntervalBounds,
    },
    surface::SurfaceSet,
};

use std::{fs::File, io, iter, path::Path};

pub struct Camera {
    // Measured in pixels
    image_width: u16,
    image_height: u16,
    // Measured in our coord system
    _viewport_width: f64,
    _viewport_height: f64,
    _focal_length: f64,
    // Additional random samples per pixel
    antialiasing: u8,
    // The Camera's location
    eye_point: Point,
    // The pixel to pixel deltas
    pixel_delta_u: Vector,
    pixel_delta_v: Vector,
    // The viewport's top-left pixel
    pixel00: Point,
    // The maximum number of ray bounces
    max_ray_bounces: u8,
}

impl Camera {
    pub fn new(image_width: u16, image_height: u16, viewport_width: f64,
        viewport_height: f64, focal_length: f64, antialiasing: u8, max_ray_bounces: u8) -> Self
    {
        let eye_point = Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let viewport_u = Vector {
            x: viewport_width,
            y: 0.0,
            z: 0.0,
        };
        let viewport_v = Vector {
            x: 0.0,
            y: -viewport_height,
            z: 0.0,
        };
        // We linearly space our image's pixels into a grid within the viewport, with
        // the gap between the viewport boundary and a pixel being half the pixel spacing
        let pixel_delta_u = viewport_u / image_width.into();
        let pixel_delta_v = viewport_v / image_height.into();
        // We take the center of the viewport to be in the -ve z direction from the eye_point, and
        // the viewoprt itself in the (x,y) plane
        let viewport_upper_left: Point = eye_point - viewport_u / 2.0 - viewport_v / 2.0 - Vector {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        };
        let pixel00 = viewport_upper_left + (pixel_delta_u + pixel_delta_v) / 2.0;
        Self {
            image_width,
            image_height,
            _viewport_width: viewport_width,
            _viewport_height: viewport_height,
            _focal_length: focal_length,
            antialiasing,
            eye_point,
            pixel_delta_u,
            pixel_delta_v,
            pixel00,
            max_ray_bounces,
        }
    }

    pub fn render(&self, world: &SurfaceSet, file_name: &Path) -> io::Result<()> {
        let vector_generator = |x: u16, y: u16| {
            let direct_ray = self.build_ray(x, y, Interval::empty());
            let diffusion = Interval::new(-0.5, 0.5, IntervalBounds::Closed);
            let vector_sum: Vector = (0..self.antialiasing)
                .map(|_| self.build_ray(x, y, diffusion))
                .chain(iter::once(direct_ray))
                .map(|ray| ray_colour(&world, ray, self.max_ray_bounces))
                .sum();
            vector_sum / (self.antialiasing as f64 + 1.0)
        };
        let image = Image::from_vectors(self.image_height, self.image_width, &vector_generator, true);
        let mut ppm_formatter = PPMFormatter::new(true);
        let mut f = File::create(file_name)?;
        image.write_to_file(&mut f, &mut ppm_formatter)
    }

    fn build_ray(&self, x: u16, y: u16, sample_space: Interval) -> Ray {
        let x = (x as f64) + sample_space.min() + sample_space.size() * rand::random::<f64>();
        let y = (y as f64) + sample_space.min() + sample_space.size() * rand::random::<f64>();
        Ray::from_two_points(
            self.eye_point,
            self.pixel00 + x * self.pixel_delta_u + y * self.pixel_delta_v
        )
    }

}


fn ray_colour(world: &SurfaceSet, ray: Ray, max_ray_bounces: u8) -> Vector {
    if max_ray_bounces == 0 {
        return Vector::zero()
    }
    let intersection = world
        .intersection(ray, Interval::new(0.001, f64::MAX, IntervalBounds::Open));
    if intersection.is_none() {
        let a = (ray.direction.normalise().y + 1.0) / 2.0;
        return (1.0 - a) * Vector::new(1.0, 1.0, 1.0) + a * Vector::new(0.5, 0.7, 1.0)
    }
    let intersection = intersection.unwrap();
    let point = ray.at(intersection.t);
    let surface = intersection.surfaces[0];
    let scattered_ray = match surface.scatter(point, ray) {
        Some(sr) => sr,
        None => return Vector::zero(),
    };
    scattered_ray.attenuation * ray_colour(world, scattered_ray.ray, max_ray_bounces - 1)
}
