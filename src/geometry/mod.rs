use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
pub type Point = Vector;

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    /// Returns the dot product of self and rhs
    /// # Example
    /// ```
    /// use ray_tracing::Point;
    /// let a = Point {
    ///     x: 1.0,
    ///     y: 2.0,
    ///     z: 3.0,
    /// };
    /// let b = Point {
    ///     x: 4.0,
    ///     y: 5.0,
    ///     z: 6.0,
    /// };
    /// assert_eq!(a.dot(b), 32.0);
    /// ```
    pub fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x +
        self.y * rhs.y +
        self.z * rhs.z
    }

    /// Returns the cross product of self and rhs
    /// # Example
    /// ```
    /// use ray_tracing::Point;
    /// let e1 = Point {
    ///     x: 1.0,
    ///     y: 0.0,
    ///     z: 0.0,
    /// };
    /// let e2 = Point {
    ///     x: 0.0,
    ///     y: 1.0,
    ///     z: 0.0,
    /// };
    /// let e3 = Point {
    ///     x: 0.0,
    ///     y: 0.0,
    ///     z: 1.0,
    /// };
    /// assert_eq!(e1.cross(e2), e3);
    /// ```
    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    /// Returns the squared L2 norm of self
    /// # Example
    /// ```
    /// use ray_tracing::Point;
    /// let a = Point {
    ///     x: 1.0,
    ///     y: 2.0,
    ///     z: 2.0,
    /// };
    /// assert_eq!(a.l2_norm_squared(), 9.0);
    /// ```
    pub fn l2_norm_squared(self) -> f64 {
        self.dot(self)
    }

    /// Returns the L2 norm of self
    /// # Example
    /// ```
    /// use ray_tracing::Point;
    /// let a = Point {
    ///     x: 1.0,
    ///     y: 2.0,
    ///     z: 2.0,
    /// };
    /// assert_eq!(a.l2_norm(), 3.0);
    /// ```
    pub fn l2_norm(self) -> f64 {
        self.l2_norm_squared().sqrt()
    }

    /// Returns a unit vector parallel to self
    /// # Example
    /// ```
    /// use ray_tracing::Point;
    /// let a = Point {
    ///     x: 1.0,
    ///     y: 2.0,
    ///     z: 2.0,
    /// };
    /// assert_eq!(a.unit(), a / 3.0);
    /// ```
    pub fn unit(self) -> Self {
        self / self.l2_norm()
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<f64> for Vector {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl Add<Vector> for f64 {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        rhs + self
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + rhs * -1.0
    }
}

impl Sub<f64> for Vector {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        self + rhs * -1.0
    }
}

impl Sub<Vector> for f64 {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        self + rhs * -1.0
    }
}

impl Mul for Vector {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}


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
