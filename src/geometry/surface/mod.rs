pub mod sphere;

use crate::geometry::{Vector, Point, Ray};

/// The trait all renderable surfaces must implement
pub trait Surface {
    /// Determines the first `Point` (if any) at which the
    /// `Ray` intersects this `Surface` for t_min <= t <= t_max
    fn intersection(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Point>;
    /// Given a `Point` on the `Surface`, return a *unit* vector normal
    /// to the `Surface` at that Point
    /// ## Undefined behaviour
    /// This trait imposes no guarantees on the method's behaviour
    /// when passed a `Point` which does not lie on the `Surface`
    fn normal(&self, point: Point) -> Vector;
}
