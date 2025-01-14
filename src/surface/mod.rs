pub mod lambertian;
pub mod metal;
pub mod dielectric;

use crate::geometry::{
    Point,
    Vector,
    UnitVector,
    Ray,
    shape::Shape,
    Interval,
    IntervalBounds,
};


/// A boundary in 3D space which scatters Rays in some (possibly random) fashion
pub trait Surface {
    /// Given a `point` on `self`, and an incident `ray`, return a
    /// random reflected `Ray`, or None if it is absorbed
    fn scatter(&self, point: Point, ray: Ray) -> Option<ScatteredRay>;
    /// Determines the first time (if any) at which `ray`
    /// intersects `self` in the `time_interval`
    fn intersection(&self, ray: Ray, time_interval: Interval) -> Option<f64>;
}

/// An attenuated, reflected `Ray`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScatteredRay {
    pub attenuation: Vector,
    pub ray: Ray,
}


/// A representation of the material of a `Shape`
pub trait Material {
    /// Given the direction of an incident ray to the material `Shape`, and the normal
    /// from the `Shape` at the point of intersection, the material should return the
    /// direction of the reflected ray, or None if it is absorbed
    /// # Parameters
    /// 1. `ray_direction` - the direction of the incident ray
    /// 1. `rebound_normal` - the normal from the Shape at the point of intersection, with
    /// convention the normal points against the incident ray
    /// 1. `entering_surface` - a closure returning true iff the ray is entering the surface, as opposed to leaving it
    /// NB: determining whether the ray is entering the surface may be expensive for some Shapes, hence the closure
    fn random_reflection(&self, ray_direction: UnitVector, rebound_normal: UnitVector, entering_surface: impl Fn() -> bool) -> Option<Reflection>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Reflection {
    pub attenuation: Vector,
    pub direction: UnitVector,
}


pub struct UniformSurface<S: Shape, M: Material> {
    shape: S,
    material: M,
}

impl<S: Shape, M: Material> UniformSurface<S, M> {
    pub fn new(shape: S, material: M) -> Self {
        Self {
            shape,
            material,
        }
    }
}

impl<S: Shape, M: Material> Surface for UniformSurface<S, M> {
    fn scatter(&self, point: Point, ray: Ray) -> Option<ScatteredRay> {
        let entering_surface = || self.shape.intersection(
            Ray::new(point, ray.direction),
            Interval::new(0.0001, f64::MAX, IntervalBounds::Open)
        ).is_some();
        let reflection = self.material.random_reflection(
            ray.direction,
            self.shape.normal_against_ray(point, ray),
            entering_surface
        )?;
        Some(ScatteredRay {
            attenuation: reflection.attenuation,
            ray: Ray {
                origin: point,
                direction: reflection.direction,
            },
        })
    }
    
    fn intersection(&self, ray: Ray, time_interval: Interval) -> Option<f64> {
        self.shape.intersection(ray, time_interval)
    }
}


pub struct SurfaceSet {
    surfaces: Vec<Box<dyn Surface>>,
}

impl SurfaceSet {
    pub fn new() -> Self {
        Self {
            surfaces: vec![],
        }
    }

    pub fn add(&mut self, surface: Box<dyn Surface>) {
        self.surfaces.push(surface);
    }

    pub fn clear(&mut self) {
        self.surfaces.clear();
    }

    /// Determines the first time (if any) at which the
    /// `Ray` intersects any `Surface` in the `time_interval`
    pub fn intersection(&self, ray: Ray, time_interval: Interval) -> Option<SurfaceSetIntersection> {
        let subsequent_bounds = match time_interval.bounds() {
            IntervalBounds::Open => IntervalBounds::LeftOpenRightClosed,
            IntervalBounds::Closed => IntervalBounds::Closed,
            IntervalBounds::LeftOpenRightClosed => IntervalBounds::LeftOpenRightClosed,
            IntervalBounds::LeftClosedRightOpen => IntervalBounds::Closed,
        };
        let mut out: Option<SurfaceSetIntersection<'_>> = None;
        self.surfaces.iter().fold(time_interval, |window, s| {
            let t = match s.intersection(ray, window) {
                Some(t) => t,
                None => return window,
            };
            if t == window.max() && out.is_some() {
                out.as_mut().unwrap().surfaces.push(s);
            } else {
                out.replace(SurfaceSetIntersection {
                    t,
                    surfaces: vec![s],
                });
            }
            Interval::new(window.min(), t, subsequent_bounds)
        });
        out
    }
}

pub struct SurfaceSetIntersection<'a> {
    pub t: f64,
    pub surfaces: Vec<&'a Box<dyn Surface>>,
}
