pub mod sphere;

use crate::geometry::{Vector, Point, Ray, Interval};

/// The trait all renderable surfaces must implement
pub trait Shape {
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
}
