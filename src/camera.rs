use crate::{
    image::{
        Pixel,
        Image,
        formatter::ppm::PPMFormatter,
    },
    geometry::{
        Point,
        Vector,
        Ray,
        Interval,
        surface::SurfaceSet,
    },
};

use std::{fs::File, path::Path, io};

pub struct Camera {
    // Measured in pixels
    pub image_width: u16,
    pub image_height: u16,
    // Measured in our coord system
    pub viewport_width: f64,
    pub viewport_height: f64,
    pub focal_length: f64,
}

impl Camera {
    pub fn render(&self, world: &SurfaceSet, file_name: &Path) -> io::Result<()> {
        let eye_point = Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        // The horizontal vector (in +ve x direction)
        let viewport_u = Vector {
            x: self.viewport_width,
            y: 0.0,
            z: 0.0,
        };
        // The vertical vector (in -ve y direction)
        let viewport_v = Vector {
            x: 0.0,
            y: -self.viewport_height,
            z: 0.0,
        };
        // We linearly space our image's pixels into a grid within the viewport, with
        // the gap between the viewport boundary and a pixel being half the pixel spacing
        // These are hence the pixel to pixel deltas
        let pixel_delta_u = viewport_u / self.image_width.into();
        let pixel_delta_v = viewport_v / self.image_height.into();
        // We take the center of the viewport to be in the -ve z direction from the eye_point, and
        // the viewoprt itself in the (x,y) plane
        let viewport_upper_left: Point = eye_point - viewport_u / 2.0 - viewport_v / 2.0 - Vector {
            x: 0.0,
            y: 0.0,
            z: self.focal_length,
        };
        let pixel00 = viewport_upper_left + (pixel_delta_u + pixel_delta_v) / 2.0;
        let colour_generator = |x: u16, y: u16| {
            let point: Point = pixel00 + (x as f64) * pixel_delta_u + (y as f64) * pixel_delta_v;
            let ray = Ray::from_two_points(eye_point, point);
            ray_colour(&world, ray)
        };
        let image = Image::new(self.image_height, self.image_width, colour_generator);
        let mut ppm_formatter = PPMFormatter::new(true);
        let mut f = File::create(file_name)?;
        image.write_to_file(&mut f, &mut ppm_formatter)
    }
}

fn ray_colour(world: &SurfaceSet, ray: Ray) -> Pixel {
    let intersection = world
        .intersection(ray, Interval::positive_reals());
    if intersection.is_none() {
        let scale_factor = (ray.direction.unit().y + 1.0) / 2.0;
        let whiteout = ((1.0 - scale_factor) * 255.0) as u8;
        return Pixel::new(whiteout, whiteout, 255)
    }
    let intersection = intersection.unwrap();
    let point = ray.at(intersection.t);
    let surface = intersection.surfaces[0];
    let colour_map = 255.0 * (1.0 + surface.outwards_normal(point)) / 2.0;
    Pixel::new(colour_map.x as u8, colour_map.y as u8, colour_map.z as u8)
}