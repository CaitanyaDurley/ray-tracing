pub mod sphere;

use crate::geometry::{Vector, Point, Ray};

/// The trait all renderable surfaces must implement
pub trait Surface {
    /// Determines the first time t (if any) at which the
    /// `Ray` intersects this `Surface` for t_min <= t <= t_max
    fn intersection(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<f64>;
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

    /// Determines the first time t (if any) at which the
    /// `Ray` intersects any `Surface` for t_min <= t <= t_max
    pub fn intersection(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<SurfaceSetIntersection> {
        let mut out = None;
        self.surfaces.iter().fold(t_max, |t, s| {
            let hit = match s.intersection(ray, t_min, t) {
                Some(p) => p,
                None => return t,
            };
            if hit < t || out.is_none() {
                out.replace(SurfaceSetIntersection {
                    t: hit,
                    surfaces: vec![s],
                });
            } else {
                out.as_mut().unwrap().surfaces.push(s)
            };
            hit
        });
        out
    }
}

pub struct SurfaceSetIntersection<'a> {
    pub t: f64,
    pub surfaces: Vec<&'a Box<dyn Surface>>,
}
