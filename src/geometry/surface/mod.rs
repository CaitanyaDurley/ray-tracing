pub mod sphere;

use crate::geometry::{Vector, Point, Ray, Interval, IntervalBounds};

/// The trait all renderable surfaces must implement
pub trait Surface {
    /// Determines the first time (if any) at which the
    /// `Ray` intersects this `Surface` in the `time_interval`
    fn intersection(&self, ray: Ray, time_interval: Interval) -> Option<f64>;
    /// Given a `Point` on the `Surface`, return the *unit* vector normal
    /// to the `Surface` at that `Point`. The normal should point out of
    /// the object defined by this surface (where this makes sense)
    /// ## Undefined behaviour
    /// This trait imposes no guarantees on the method's behaviour
    /// when passed a `Point` which does not lie on the `Surface`
    fn outwards_normal(&self, point: Point) -> Vector;
    /// Identical to `outwards_normal`, except the unit vector will point
    /// "against" the incident ray, rather than out of the surface
    fn normal_against_ray(&self, point: Point, ray: Ray) -> Vector {
        let n = self.outwards_normal(point);
        - n.dot(ray.direction).signum() * n
    }
    /// Given a `Point` on the `Surface` and an incident `Ray`, return
    /// a random reflection from that Point. The reflected Ray's direction
    /// is guaranteed to be of unit length and has a distribution
    /// proportional to the cosine of the angle between the incident ray
    /// and the normal (against the ray) at the point of intersection.
    fn random_reflection(&self, point: Point, ray: Ray) -> Ray {
        let direction = self.normal_against_ray(point, ray) +
            Vector::random_unit();
        Ray {
            origin: point,
            direction,
        }
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
        let subsequent_bounds = match time_interval.bounds {
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
            if t == window.max && out.is_some() {
                out.as_mut().unwrap().surfaces.push(s);
            } else {
                out.replace(SurfaceSetIntersection {
                    t,
                    surfaces: vec![s],
                });
            }
            Interval::new(window.min, t, subsequent_bounds)
        });
        out
    }
}

pub struct SurfaceSetIntersection<'a> {
    pub t: f64,
    pub surfaces: Vec<&'a Box<dyn Surface>>,
}
