use super::*;

/// A Dielectric material always refracts the incident ray according to
/// its refraction index
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn random_reflection(&self, ray_direction: Vector, rebound_normal: Vector, entering_surface: impl Fn() -> bool) -> Option<Reflection> {
        let relative_index = if entering_surface() {
            self.refraction_index
        } else {
            1.0 / self.refraction_index
        };
        let r = ray_direction.normalise();
        let refracted_perpendicular = 1.0 / relative_index * (
            r - r.dot(rebound_normal) * rebound_normal
        );
        let refracted_parallel = -1.0 * rebound_normal * f64::sqrt(
            1.0 - refracted_perpendicular.l2_norm_squared()
        );
        Some(Reflection {
            attenuation: Vector::new(1.0, 1.0, 1.0),
            direction: refracted_parallel + refracted_perpendicular,
        })
    }
}
