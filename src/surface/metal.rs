use super::*;

/// A Metal material perfectly reflects an incident ray such
/// that the angle between the point of incidence and the normal
/// (against the ray) is preserved.
pub struct Metal {
    albedo: Vector,
}

impl Metal {
    pub fn new(albedo: Vector) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn random_reflection(&self, ray_direction: Vector, outwards_normal: Vector) -> Option<Reflection> {
        Some(Reflection {
            attenuation: self.albedo,
            direction: ray_direction - 2.0 * outwards_normal * ray_direction.dot(outwards_normal),
        })
    }
}
