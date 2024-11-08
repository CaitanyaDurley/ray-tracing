mod vector;
pub mod surface;

pub use vector::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
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
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    /// Returns the interval containing all reals [-infty, +infty]
    pub fn all_reals() -> Self {
        Self {
            min: f64::MIN,
            max: f64::MAX,
        }
    }

    /// Returns the interval containing all positive reals [0, +infty]
    pub fn positive_reals() -> Self {
        Self {
            min: 0.0,
            max: f64::MAX,
        }
    }

    /// Returns the empty interval [0.0, 0.0]
    pub fn empty() -> Self {
        Self {
            min: 0.0,
            max: 0.0,
        }
    }

    /// Returns the size of the `Interval`
    /// # Example
    /// ```
    /// use ray_tracing::Interval;
    /// let interval = Interval {
    ///     min: 3.0,
    ///     max: 5.5,
    /// };
    /// assert_eq!(interval.size(), 2.5)
    /// ```
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    /// Tests if `num` lies within the *closed* `Interval`
    /// # Example
    /// ```
    /// use ray_tracing::Interval;
    /// let interval = Interval {
    ///     min: 3.0,
    ///     max: 5.5,
    /// };
    /// assert!(interval.contains(4.0));
    /// assert!(interval.contains(5.5));
    /// ```
    pub fn contains(&self, num: f64) -> bool {
        self.min <= num && num <= self.max
    }

        /// Tests if `num` lies within the *open* `Interval`
    /// # Example
    /// ```
    /// use ray_tracing::Interval;
    /// let interval = Interval {
    ///     min: 3.0,
    ///     max: 5.5,
    /// };
    /// assert!(interval.contains_strict(4.0));
    /// assert!(!interval.contains_strict(5.5));
    /// ```
    pub fn contains_strict(&self, num: f64) -> bool {
        self.min < num && num < self.max
    }
}
