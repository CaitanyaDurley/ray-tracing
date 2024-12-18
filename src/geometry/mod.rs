mod vector;
pub mod shape;

pub use vector::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn from_two_points(origin: Point, second_point: Point) -> Self {
        Self {
            origin,
            direction: second_point - origin,
        }
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IntervalBounds {
    Open,
    Closed,
    LeftOpenRightClosed,
    LeftClosedRightOpen,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interval {
    min: f64,
    max: f64,
    bounds: IntervalBounds,
}

impl Interval {
    pub fn new(min: f64, max: f64, bounds: IntervalBounds) -> Self {
        Self {
            min,
            max,
            bounds,
        }
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn max(&self) -> f64 {
        self.max
    }

    pub fn bounds(&self) -> IntervalBounds {
        self.bounds
    }

    /// Returns an interval with bounds (f64::MIN, f64::MAX)
    pub fn all_reals(bounds: IntervalBounds) -> Self {
        Self {
            min: f64::MIN,
            max: f64::MAX,
            bounds,
        }
    }

    /// Returns an interval with bounds (0.0, f64::MAX)
    pub fn positive_reals(bounds: IntervalBounds) -> Self {
        Self {
            min: 0.0,
            max: f64::MAX,
            bounds,
        }
    }

    /// Returns the empty interval (0.0, 0.0)
    pub fn empty() -> Self {
        Self {
            min: 0.0,
            max: 0.0,
            bounds: IntervalBounds::Open,
        }
    }

    /// Returns the size of the `Interval`
    /// # Example
    /// ```
    /// use ray_tracing::{ Interval, IntervalBounds };
    /// let interval = Interval::new(3.0, 5.5, IntervalBounds::Open);
    /// assert_eq!(interval.size(), 2.5)
    /// ```
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    /// Tests if `num` lies within the `Interval`
    /// # Example
    /// ```
    /// use ray_tracing::{ Interval, IntervalBounds };
    /// let interval = Interval::new(3.0, 5.5, IntervalBounds::Open);
    /// assert!(interval.contains(4.0));
    /// assert!(!interval.contains(5.5));
    /// ```
    pub fn contains(&self, num: f64) -> bool {
        match self.bounds {
            IntervalBounds::Open => self.min < num && num < self.max,
            IntervalBounds::Closed => self.min <= num && num <= self.max,
            IntervalBounds::LeftOpenRightClosed => self.min < num && num <= self.max,
            IntervalBounds::LeftClosedRightOpen => self.min <= num && num < self.max,
        }
    }
}
