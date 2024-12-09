mod image;
mod geometry;
mod camera;

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
        surface::{
            Surface,
            SurfaceSet,
            SurfaceSetIntersection,
            sphere::Sphere,
        },
    },
    camera::Camera,
};
