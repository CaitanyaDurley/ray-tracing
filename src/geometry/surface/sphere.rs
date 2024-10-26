use crate::geometry::surface::{
    Surface,
    Vector,
    Point,
    Ray,
};


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

impl Surface for Sphere {
    fn intersection(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Point> {
        let oc = self.center - ray.origin;
        let a = ray.direction.l2_norm_squared();
        let h = ray.direction.dot(oc);
        let c = oc.l2_norm_squared() - self.radius.powi(2);
        let discriminant = h.powi(2) - a * c;
        if discriminant < 0.0 {
            return None
        }
        let discriminant_sqrt = discriminant.sqrt();
        let t = [-1.0, 1.0].into_iter()
            .map(|s| (h + s * discriminant_sqrt) / a)
            .filter(|t| t_min <= *t && *t <= t_max)
            .next();
        t.and_then(|x| Some(ray.at(x)))
    }

    fn normal(&self, point: Point) -> Vector {
        (point - self.center) / self.radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(sphere.intersection(ray, 0.0, f64::MAX), None);
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
            sphere.intersection(ray, 0.0, f64::MAX),
            Some(Point::new(2.0, 0.0, 0.0))
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
            sphere.intersection(ray, 0.0, f64::MAX),
            Some(Point::new(1.0, 0.0, 0.0))
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
        assert_eq!(
            sphere.intersection(ray, 1.5, f64::MAX),
            Some(Point::new(3.0, 0.0, 0.0))
        );
    }

    #[test]
    fn normal_is_unit() {
        let sphere = Sphere::new(
            Point::new(0.0, 0.0, 0.0),
            5.0,
        );
        let point = Point::new(3.0, 4.0, 0.0);
        assert_eq!(sphere.normal(point).l2_norm(), 1.0);
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
        assert_eq!(sphere.normal(point).dot(e2), 0.0);
        assert_eq!(sphere.normal(point).dot(e3), 0.0);
    }
}