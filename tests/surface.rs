use ray_tracing::{Point, Ray, Surface, SurfaceSet, Vector};

struct DummySurface {
    border: f64,
}

impl Surface for DummySurface {
    fn intersection(&self, _ray: Ray, t_min: f64, t_max: f64) -> Option<f64> {
        if t_min <= self.border && self.border <= t_max {
            Some(self.border)
        } else {
            None
        }
    }

    fn outwards_normal(&self, _point: Point) -> Vector {
        Vector::new(1.0, 0.0, 0.0)
    }
}

#[test]
fn ray_in_direction_of_outwards_normal() {
    let surface = DummySurface {
        border: 3.0,
    };
    let origin = Point::new(0.0, 0.0, 0.0);
    let ray = Ray {
        origin,
        direction: Vector::new(2.0, 3.0, 4.0),
    };
    assert_eq!(
        surface.normal_against_ray(origin, ray),
        Vector::new(-1.0, 0.0, 0.0),
    );
}

#[test]
fn ray_opposite_direction_of_outwards_normal() {
    let surface = DummySurface {
        border: 3.0,
    };
    let origin = Point::new(0.0, 0.0, 0.0);
    let ray = Ray {
        origin,
        direction: Vector::new(-2.0, 3.0, 4.0),
    };
    assert_eq!(
        surface.normal_against_ray(origin, ray),
        Vector::new(1.0, 0.0, 0.0),
    );
}

#[test]
fn surface_set_intersection_returns_first() {
    let first_surface = Box::new(DummySurface {
        border: 2.0,
    });
    let second_surface = Box::new(DummySurface {
        border: 3.0,
    });
    let mut surface_set = SurfaceSet::new();
    surface_set.add(first_surface);
    surface_set.add(second_surface);
    let ray = Ray {
        origin: Point::new(0.0, 0.0, 0.0),
        direction: Vector::new(1.0, 0.0, 0.0),
    };
    let surface_set_intersection = surface_set.intersection(ray, 0.0, 4.0).unwrap();
    assert_eq!(surface_set_intersection.t, 2.0);
    assert_eq!(surface_set_intersection.surfaces.len(), 1);
}
