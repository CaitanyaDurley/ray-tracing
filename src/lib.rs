mod image;
mod geometry;
mod camera;
mod surface;

pub use self::{
    image::{
        Pixel,
        Image,
        formatter::{
            ImageFormatter,
            ppm::PPMFormatter,
        },
    },
    geometry::{
        Point,
        Vector,
        Ray,
        Interval,
        IntervalBounds,
        shape::{
            Shape,
            sphere::Sphere,
        },
    },
    camera::Camera,
    surface::{
        Reflection,
        Material,
        UniformSurface,
        SurfaceSet,
        lambertian::Lambertian,
    },
};
