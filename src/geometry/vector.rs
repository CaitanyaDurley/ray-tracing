use core::f64::consts::PI;
use std::{
    cmp::Ordering,
    convert::identity,
    iter::Sum,
    ops::{Add, Deref, Div, Mul, Sub}
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
    /// assert_eq!(a.normalise(), a / 3.0);
    /// ```
    pub fn normalise(self) -> Self {
        self / self.l2_norm()
    }

    /// Apply the closure against each element in the `Vector`
    /// # Example
    /// ```
    /// use ray_tracing::Vector;
    /// let a = Vector::new(1.0, 4.0, 9.0);
    /// let expected = Vector::new(1.0, 2.0, 3.0);
    /// assert_eq!(a.map(f64::sqrt), expected);
    /// ```
    pub fn map<F: Fn(f64) -> f64>(self, f: F) -> Self {
        Self::new(f(self.x), f(self.y), f(self.z))
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



#[derive(Debug, Copy, Clone, PartialEq)]
/// A *unit* vector, implementing `Into` and `Deref` such that
/// it may be used interchangeably with `Vector`
pub struct UnitVector(Vector);


impl UnitVector {
    /// Create a unit vector given its incline and rotation
    /// # Arguments
    /// 1. `incline` - the angle (in radians) to the x-y plane
    /// 1. `rotation` - the angle (in radians) within the x-y plane
    /// # Example
    /// ```
    /// use ray_tracing::{Vector, UnitVector};
    /// use core::f64::consts::PI;
    /// let v = UnitVector::new(-PI / 2.0, PI);
    /// let v2 = Vector::new(0.0, 0.0, -1.0);
    /// let diff = v.to_vector() - v2;
    /// assert!(diff.l2_norm().abs() < 1e-12);
    /// ```
    pub fn new(incline: f64, rotation: f64) -> Self {
        let hypotenuse = incline.cos();
        Self(Vector::new(
            hypotenuse * rotation.cos(),
            hypotenuse * rotation.sin(),
            incline.sin()
        ))
    }

    /// Create the unit vector parallel to vector
    /// # Parameters
    /// `vector` - need not be of unit length
    /// # Example
    /// ```
    /// use ray_tracing::{Vector, UnitVector};
    /// let actual = UnitVector::from(Vector::new(2.0, 0.0, 0.0));
    /// let expected = UnitVector::new(0.0, 0.0);
    /// assert_eq!(actual, expected);
    /// ```
    pub fn from(vector: Vector) -> Self {
        Self(vector.normalise())
    }

    /// Convert the UnitVector into a regular Vector
    /// # Example
    /// ```
    /// use ray_tracing::{Vector, UnitVector};
    /// let v1 = Vector::new(1.0, 0.0, 0.0);
    /// let v2 = UnitVector::new(0.0, 0.0).to_vector();
    /// assert_eq!(v1, v2)
    /// ```
    pub fn to_vector(self) -> Vector {
        self.0
    }

    /// Returns a random unit vector
    /// # Example
    /// ```
    /// use ray_tracing::UnitVector;
    /// let v = UnitVector::random();
    /// assert!((v.l2_norm() - 1.0).abs() < 1e-12);
    /// ```
    pub fn random() -> Self {
        let incline = 2.0 * PI * rand::random::<f64>();
        let rot = 2.0 * PI * rand::random::<f64>();
        Self::new(incline, rot)
    }
}


impl Deref for UnitVector {
    type Target = Vector;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl From<UnitVector> for Vector {
    fn from(value: UnitVector) -> Self {
        value.to_vector()
    }
}


impl Add for UnitVector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        self.to_vector() + rhs.to_vector()
    }
}


impl Add<f64> for UnitVector {
    type Output = Vector;

    fn add(self, rhs: f64) -> Self::Output {
        self.to_vector() + rhs
    }
}


impl Add<UnitVector> for f64 {
    type Output = Vector;

    fn add(self, rhs: UnitVector) -> Self::Output {
        rhs + self
    }
}


impl Add<Vector> for UnitVector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        self.to_vector() + rhs
    }
}


impl Add<UnitVector> for Vector {
    type Output = Self;

    fn add(self, rhs: UnitVector) -> Self::Output {
        self + rhs.to_vector()
    }
}


impl Sub for UnitVector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        self.to_vector() - rhs.to_vector()
    }
}


impl Sub<f64> for UnitVector {
    type Output = Vector;

    fn sub(self, rhs: f64) -> Self::Output {
        self.to_vector() - rhs
    }
}


impl Sub<UnitVector> for f64 {
    type Output = Vector;

    fn sub(self, rhs: UnitVector) -> Self::Output {
        self - rhs.to_vector()
    }
}


impl Sub<Vector> for UnitVector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        self.to_vector() - rhs
    }
}


impl Sub<UnitVector> for Vector {
    type Output = Self;

    fn sub(self, rhs: UnitVector) -> Self::Output {
        self - rhs.to_vector()
    }
}


impl Mul for UnitVector {
    type Output = Vector;

    fn mul(self, rhs: Self) -> Self::Output {
        self.to_vector() * rhs.to_vector()
    }
}


impl Mul<f64> for UnitVector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        self.to_vector() * rhs
    }
}


impl Mul<Vector> for UnitVector {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        self.to_vector() * rhs
    }
}


impl Mul<UnitVector> for Vector {
    type Output = Self;

    fn mul(self, rhs: UnitVector) -> Self::Output {
        self * rhs.to_vector()
    }
}


impl Mul<UnitVector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: UnitVector) -> Self::Output {
        self * rhs.to_vector()
    }
}


impl Div<f64> for UnitVector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        self.to_vector() / rhs
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