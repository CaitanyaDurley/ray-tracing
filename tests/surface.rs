use ray_tracing::{Interval, IntervalBounds, Material, Point, Ray, Reflection, Shape, SurfaceSet, UniformSurface, Vector};

struct DummyShape {
    border: f64,
}

impl Shape for DummyShape {
    fn intersection(&self, _ray: Ray, time_interval: Interval) -> Option<f64> {
        time_interval
            .contains(self.border)
            .then_some(self.border)
    }

    fn outwards_normal(&self, _point: Point) -> Vector {
        Vector::new(1.0, 0.0, 0.0)
    }
}

struct DummyMaterial {}

impl Material for DummyMaterial {
    fn random_reflection(&self, _ray_direction: Vector, rebound_normal: Vector, _entering_surface: impl Fn() -> bool) -> Option<Reflection> {
        Some(Reflection {
            attenuation: Vector::zero(),
            direction: rebound_normal,
        })
    }
}

type DummySurface = UniformSurface<DummyShape, DummyMaterial>;


#[test]
fn normal_against_ray_in_direction_of_outwards_normal() {
    let shape = DummyShape {
        border: 3.0,
    };
    let origin = Point::new(0.0, 0.0, 0.0);
    let ray = Ray {
        origin,
        direction: Vector::new(2.0, 3.0, 4.0),
    };
    assert_eq!(
        shape.normal_against_ray(origin, ray),
        Vector::new(-1.0, 0.0, 0.0),
    );
}

#[test]
fn normal_against_ray_in_opposite_direction_of_outwards_normal() {
    let shape = DummyShape {
        border: 3.0,
    };
    let origin = Point::new(0.0, 0.0, 0.0);
    let ray = Ray {
        origin,
        direction: Vector::new(-2.0, 3.0, 4.0),
    };
    assert_eq!(
        shape.normal_against_ray(origin, ray),
        Vector::new(1.0, 0.0, 0.0),
    );
}

#[test]
fn surface_set_intersection_returns_first() {
    let first_shape = DummyShape {
        border: 2.0,
    };
    let second_shape = DummyShape {
        border: 3.0,
    };
    let mut surface_set = SurfaceSet::new();
    surface_set.add(Box::new(DummySurface::new(first_shape, DummyMaterial {})));
    surface_set.add(Box::new(DummySurface::new(second_shape, DummyMaterial {})));
    let ray = Ray {
        origin: Point::new(0.0, 0.0, 0.0),
        direction: Vector::new(1.0, 0.0, 0.0),
    };
    let surface_set_intersection = surface_set
        .intersection(ray, Interval::positive_reals(IntervalBounds::Open))
        .unwrap();
    assert_eq!(surface_set_intersection.t, 2.0);
    assert_eq!(surface_set_intersection.surfaces.len(), 1);
}
