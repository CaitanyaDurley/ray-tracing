pub mod sphere;

use crate::geometry::{Vector, Point, Ray};

/// The trait all renderable surfaces must implement
pub trait Surface {
    /// Determines the first `Point` (if any) at which the
    /// `Ray` intersects this `Surface` for t_min <= t <= t_max
    fn intersection(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Point>;
    /// Given a `Point` on the `Surface`, return the *unit* vector normal
    /// to the `Surface` at that `Point`. The normal should point out of
    /// the object defined by this surface (where this makes sense)
    /// ## Undefined behaviour
    /// This trait imposes no guarantees on the method's behaviour
    /// when passed a `Point` which does not lie on the `Surface`
    fn outwards_normal(&self, point: Point) -> Vector;
    /// Identical to `normal`, except the unit vector will point
    /// "against" the incident ray, rather than out of the surface
    fn normal_against_ray(&self, point: Point, ray: Ray) -> Vector {
        let outwards_normal = self.outwards_normal(point);
        outwards_normal * if ray.direction.dot(outwards_normal) > 0.0 {
            -1.0
        } else {
            1.0
        }
    }
}
