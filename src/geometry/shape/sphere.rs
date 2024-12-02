use super::*;


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Self {
        assert!(radius > 0.0);
        Self {
            center,
            radius,
        }
    }
}

impl Shape for Sphere {
    fn intersection(&self, ray: Ray, time_interval: Interval) -> Option<f64> {
        let oc = self.center - ray.origin;
        let a = ray.direction.l2_norm_squared();
        let h = ray.direction.dot(oc);
        let c = oc.l2_norm_squared() - self.radius.powi(2);
        let discriminant = h.powi(2) - a * c;
        if discriminant < 0.0 {
            return None
        }
        let discriminant_sqrt = discriminant.sqrt();
        [-1.0, 1.0].into_iter()
            .map(|s| (h + s * discriminant_sqrt) / a)
            .filter(|t| time_interval.contains(*t))
            .next()
    }

    fn outwards_normal(&self, point: Point) -> Vector {
        (point - self.center) / self.radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::IntervalBounds;

    #[test]
    #[should_panic(expected = "assertion failed: radius > 0.0")]
    fn negative_radius_panics() {
        Sphere::new(Point::new(0.0, 0.0, 0.0), -1.0);
    }

    #[test]
    fn non_intersection_returns_none() {
        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(1.0, 0.0, 0.0),
        };
        let sphere = Sphere::new(
            Point::new(-2.0, 0.0, 0.0),
            1.0,
        );
        assert_eq!(
            sphere.intersection(ray, Interval::positive_reals(IntervalBounds::Open)),
            None
        );
    }

    #[test]
    fn tangent_returns_solution() {
        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(1.0, 0.0, 0.0),
        };
        let sphere = Sphere::new(
            Point::new(2.0, 1.0, 0.0),
            1.0,
        );
        assert_eq!(
            sphere.intersection(ray, Interval::positive_reals(IntervalBounds::Open)),
            Some(2.0)
        );
    }

    #[test]
    fn two_solutions_returns_earlier() {
        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(1.0, 0.0, 0.0),
        };
        let sphere = Sphere::new(
            Point::new(2.0, 0.0, 0.0),
            1.0,
        );
        assert_eq!(
            sphere.intersection(ray, Interval::positive_reals(IntervalBounds::Open)),
            Some(1.0)
        );
    }

    #[test]
    fn two_solutions_one_in_range() {
        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(1.0, 0.0, 0.0),
        };
        let sphere = Sphere::new(
            Point::new(2.0, 0.0, 0.0),
            1.0,
        );
        let window = Interval::new(1.0, 3.0, IntervalBounds::LeftOpenRightClosed);
        assert_eq!(sphere.intersection(ray, window), Some(3.0));
    }

    #[test]
    fn normal_is_unit() {
        let sphere = Sphere::new(
            Point::new(0.0, 0.0, 0.0),
            5.0,
        );
        let point = Point::new(3.0, 4.0, 0.0);
        assert_eq!(sphere.outwards_normal(point).l2_norm(), 1.0);
    }

    #[test]
    fn normal_is_perpendicular() {
        let sphere = Sphere::new(
            Point::new(0.0, 0.0, 0.0),
            1.0,
        );
        let point = Point::new(1.0, 0.0, 0.0);
        let e2 = Vector::new(0.0, 1.0, 0.0);
        let e3 = Vector::new(0.0, 0.0, 1.0);
        assert_eq!(sphere.outwards_normal(point).dot(e2), 0.0);
        assert_eq!(sphere.outwards_normal(point).dot(e3), 0.0);
    }
}