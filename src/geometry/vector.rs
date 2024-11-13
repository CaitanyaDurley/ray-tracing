use std::{
    cmp::Ordering,
    convert::identity,
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

    /// Returns a random vector uniformly distributed in [low, high)^3
    /// # Example
    /// ```
    /// use ray_tracing::Vector;
    /// let v = Vector::random_within(0.0, 1.0);
    /// assert!(0.0 <= v && v < 1.0)
    /// ```
    pub fn random_within(low: f64, high: f64) -> Self {
        low + (high - low) * Vector::new(rand::random(), rand::random(), rand::random())
    }

    /// Returns a random unit vector
    /// # Example
    /// ```
    /// use ray_tracing::Vector;
    /// let v = Vector::random_unit();
    /// assert!((v.l2_norm() - 1.0).abs() < 1e-12);
    /// ```
    pub fn random_unit() -> Self {
        let incline = 2.0 * 3.141 * rand::random::<f64>();
        let hypotenuse = incline.cos();
        let rot = 2.0 * 3.141 * rand::random::<f64>();
        Self::new(hypotenuse * rot.cos(), hypotenuse * rot.sin(), incline.sin())
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


impl PartialEq<f64> for Vector {
    fn eq(&self, other: &f64) -> bool {
        let other = *other;
        self.x == other && self.y == other && self.z == other
    }
}


impl PartialEq<Vector> for f64 {
    fn eq(&self, other: &Vector) -> bool {
        other.eq(self)
    }
}


impl PartialOrd for Vector {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        [self.x, self.y, self.z].into_iter()
            .zip([other.x, other.y, other.z])
            .map(|(s, o)| s.partial_cmp(&o))
            .reduce(|a, e| if a == e {
                a
            } else {
                None
            })
            .unwrap()
    }
    
    fn le(&self, other: &Self) -> bool {
        [self.x, self.y, self.z].into_iter()
            .zip([other.x, other.y, other.z])
            .map(|(s, o)| s <= o)
            .all(identity)
    }
    
    fn ge(&self, other: &Self) -> bool {
        [self.x, self.y, self.z].into_iter()
            .zip([other.x, other.y, other.z])
            .map(|(s, o)| s >= o)
            .all(identity)
    }    
}


impl PartialOrd<f64> for Vector {
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        self.partial_cmp(&Vector::new(*other, *other, *other))
    }
    
    fn le(&self, other: &f64) -> bool {
        self.le(&Vector::new(*other, *other, *other))
    }
    
    fn ge(&self, other: &f64) -> bool {
        self.ge(&Vector::new(*other, *other, *other))
    }
}


impl PartialOrd<Vector> for f64 {
    fn partial_cmp(&self, other: &Vector) -> Option<Ordering> {
        Vector::new(*self, *self, *self).partial_cmp(other)
    }
    
    fn le(&self, other: &Vector) -> bool {
        other.ge(self)
    }
    
    fn ge(&self, other: &Vector) -> bool {
        other.le(self)
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

    #[test]
    fn vector_eq_vector() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = a.clone();
        assert_eq!(a, b);
    }

    #[test]
    fn vector_ne_vector() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = a + 1.0;
        assert_ne!(a, b);
    }

    #[test]
    fn vector_eq_float() {
        let a = Vector::new(1.0, 1.0, 1.0);
        assert_eq!(a, 1.0);
    }

    #[test]
    fn vector_ne_float() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = 1.0;
        assert_ne!(a, b);
    }

    #[test]
    fn vector_lt_vector() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = a - 1.0;
        assert!(b < a)
    }

    #[test]
    fn vector_le_vector() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(1.0, 1.9, 3.0);
        assert!(b <= a)
    }

    #[test]
    fn vector_gt_vector() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = a + 1.0;
        assert!(b > a)
    }

    #[test]
    fn vector_ge_vector() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(1.0, 2.1, 3.0);
        assert!(b >= a)
    }

    #[test]
    fn vector_lt_float() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = 4.0;
        assert!(a < b)
    }

    #[test]
    fn vector_le_float() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = 3.0;
        assert!(a <= b)
    }

    #[test]
    fn vector_gt_float() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = 0.0;
        assert!(a > b)
    }

    #[test]
    fn vector_ge_float() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = 1.0;
        assert!(a >= b)
    }

    #[test]
    fn float_lt_vector() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = 0.0;
        assert!(b < a)
    }

    #[test]
    fn float_le_vector() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = 1.0;
        assert!(b <= a)
    }

    #[test]
    fn float_gt_vector() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = 4.0;
        assert!(b > a)
    }

    #[test]
    fn float_ge_vector() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = 3.0;
        assert!(b >= a)
    }
}