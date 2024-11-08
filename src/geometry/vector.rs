use std::{
    iter::Sum,
    ops::{Add, Div, Mul, Sub}
};


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

    pub fn zero() -> Self {
        Vector::new(0.0, 0.0, 0.0)
    }

    /// Returns the dot product of self and rhs
    /// # Example
    /// ```
    /// use ray_tracing::Vector;
    /// let a = Vector::new(1.0, 2.0, 3.0);
    /// let b = Vector::new(4.0, 5.0, 6.0);
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
    /// use ray_tracing::Vector;
    /// let e1 = Vector::new(1.0, 0.0, 0.0);
    /// let e2 = Vector::new(0.0, 1.0, 0.0);
    /// let e3 = Vector::new(0.0, 0.0, 1.0);
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
    /// use ray_tracing::Vector;
    /// let a = Vector::new(1.0, 2.0, 2.0);
    /// assert_eq!(a.l2_norm_squared(), 9.0);
    /// ```
    pub fn l2_norm_squared(self) -> f64 {
        self.dot(self)
    }

    /// Returns the L2 norm of self
    /// # Example
    /// ```
    /// use ray_tracing::Vector;
    /// let a = Vector::new(1.0, 2.0, 2.0);
    /// assert_eq!(a.l2_norm(), 3.0);
    /// ```
    pub fn l2_norm(self) -> f64 {
        self.l2_norm_squared().sqrt()
    }

    /// Returns a unit vector parallel to self
    /// # Example
    /// ```
    /// use ray_tracing::Vector;
    /// let a = Vector::new(1.0, 2.0, 2.0);
    /// assert_eq!(a.to_unit(), a / 3.0);
    /// ```
    pub fn to_unit(self) -> Self {
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


impl Sum for Vector {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vector::zero(), |a, b| a + b)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_plus_vector() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(4.0, 5.0, 6.0);
        assert_eq!(a + b, Vector::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn vector_plus_float() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let f = 10.0;
        assert_eq!(v + f, Vector::new(11.0, 12.0, 13.0))
    }

    #[test]
    fn float_plus_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let f = 10.0;
        assert_eq!(f + v, Vector::new(11.0, 12.0, 13.0))
    }

    #[test]
    fn vector_minus_vector() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(4.0, 5.0, 6.0);
        assert_eq!(a - b, Vector::new(-3.0, -3.0, -3.0));
    }

    #[test]
    fn vector_minus_float() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let f = 10.0;
        assert_eq!(v - f, Vector::new(-9.0, -8.0, -7.0))
    }

    #[test]
    fn float_minus_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let f = 10.0;
        assert_eq!(f - v, Vector::new(9.0, 8.0, 7.0))
    }

    #[test]
    fn vector_times_vector() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(4.0, 5.0, 6.0);
        assert_eq!(a * b, Vector::new(4.0, 10.0, 18.0));
    }

    #[test]
    fn vector_times_float() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let f = 10.0;
        assert_eq!(v * f, Vector::new(10.0, 20.0, 30.0))
    }

    #[test]
    fn float_times_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let f = 10.0;
        assert_eq!(f * v, Vector::new(10.0, 20.0, 30.0))
    }

    #[test]
    fn vector_divided_by_float() {
        let v = Vector::new(1.0, 2.0, 4.0);
        let f = 10.0;
        assert_eq!(v / f, Vector::new(0.1, 0.2, 0.4))
    }

    #[test]
    fn sum_iterator_of_vectors() {
        let s: Vector = [
            Vector::new(1.0, 2.0, 3.0),
            Vector::new(4.0, 5.0, 6.0),
            Vector::new(7.0, 8.0, 9.0),
        ].into_iter().sum();
        assert_eq!(s, Vector::new(12.0, 15.0 , 18.0))
    }
}