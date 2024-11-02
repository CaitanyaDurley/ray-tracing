mod image;
mod geometry;

pub use image::{
    Pixel,
    Image,
    formatter::{
        ImageFormatter,
        ppm::PPMFormatter,
    },
};

pub use geometry::{
    Point,
    Vector,
    Ray,
    Interval,
    surface::{
        Surface,
        SurfaceSet,
        SurfaceSetIntersection,
        sphere::Sphere,
    },
};
