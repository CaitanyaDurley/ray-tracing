use super::*;

/// A Lambertian material scatters a ray in a random direction
/// from the point of incidence. The reflected ray's direction
/// has a distribution proportional to the cosine of the angle
/// between the incident ray and the normal (against the ray)
/// at the point of intersection.
pub struct Lambertian {
    albedo: Vector,
}

impl Lambertian {
    pub fn new(albedo: Vector) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn random_reflection(&self, _ray_direction: Vector, outwards_normal: Vector) -> Option<Reflection> {
        Some(Reflection {
            attenuation: self.albedo,
            direction: outwards_normal + Vector::random_unit(),
        })
    }
}
