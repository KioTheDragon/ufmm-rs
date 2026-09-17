use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

use super::Scalar;
use super::Vector3;
use super::Vector3Interop;

///
/// define struct Cartesian3
///
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cartesian3<F> {
    pub x: F,
    pub y: F,
    pub z: F,
}

impl<F: Scalar> Cartesian3<F> {
    pub fn new(x: F, y: F, z: F) -> Self {
        Self { x, y, z }
    }
}

///
/// default values
///
impl<F: Scalar> Default for Cartesian3<F> {
    fn default() -> Self {
        Self::zero()
    }
}
impl<F: Scalar> Cartesian3<F> {
    // (1, 0, 0)
    pub fn x_axis() -> Self {
        Self::new(F::one(), F::zero(), F::zero())
    }
    // (0, 1, 0)
    pub fn y_axis() -> Self {
        Self::new(F::zero(), F::one(), F::zero())
    }
    // (0, 0, 1)
    pub fn z_axis() -> Self {
        Self::new(F::zero(), F::zero(), F::one())
    }
}

///
/// basic math operations
///
// vec + vec
impl<F: Scalar> Add for Cartesian3<F> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

// vec += vec
impl<F: Scalar> AddAssign for Cartesian3<F> {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

// vec - vec
impl<F: Scalar> Sub for Cartesian3<F> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

// vec -= vec
impl<F: Scalar> SubAssign for Cartesian3<F> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
    }
}

// -vec
impl<F: Scalar> Neg for Cartesian3<F> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

// vec * scalar
impl<F: Scalar> Mul<F> for Cartesian3<F> {
    type Output = Self;
    fn mul(self, rhs: F) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

// vec *= scalar
impl<F: Scalar> MulAssign<F> for Cartesian3<F> {
    fn mul_assign(&mut self, rhs: F) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}

// vec / scalar
impl<F: Scalar> Div<F> for Cartesian3<F> {
    type Output = Self;
    fn div(self, rhs: F) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

// vec /= scalar
impl<F: Scalar> DivAssign<F> for Cartesian3<F> {
    fn div_assign(&mut self, rhs: F) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
    }
}

///
/// impl Vector3 trait
///
impl<F: Scalar> Vector3<F> for Cartesian3<F> {
    fn zero() -> Self {
        Self::new(F::zero(), F::zero(), F::zero())
    }
    fn dot(self, rhs: Cartesian3<F>) -> F {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    fn cross(self, rhs: Cartesian3<F>) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
    fn length_squared(&self) -> F {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    fn length(&self) -> F {
        self.length_squared().sqrt()
    }
    fn normalize(&mut self) {
        let len = self.length();
        if len.is_zero() {
            *self = Self::zero();
        } else {
            *self /= len;
        }
    }
    fn normalized(&self) -> Self {
        let len = self.length();
        if len.is_zero() {
            Self::zero()
        } else {
            *self / len
        }
    }
    fn distance(&self, rhs: &Self) -> F {
        (*self - *rhs).length()
    }
    fn distance_squared(&self, rhs: &Self) -> F {
        (*self - *rhs).length_squared()
    }
    fn angle_between(&self, rhs: &Self) -> F {
        let denom = self.length() * rhs.length();
        if denom.is_zero() {
            return F::zero();
        }
        let cos = (self.dot(*rhs) / denom).clamp(-F::one(), F::one());
        cos.acos()
    }
}

// vec[i]
impl<F: Scalar> Index<usize> for Cartesian3<F> {
    type Output = F;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bounds: the len is 3 but the index is {index}"),
        }
    }
}

// vec[i] = . . .
impl<F: Scalar> IndexMut<usize> for Cartesian3<F> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of bounds: the len is 3 but the index is {index}"),
        }
    }
}

// sum of [vec]
impl<F: Scalar> Sum for Cartesian3<F> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |acc, v| acc + v)
    }
}

/// guaranty about conversions
impl<F: Scalar> Vector3Interop<F> for Cartesian3<F> {}

///
/// mint conversions
///
impl<F: Scalar> From<mint::Vector3<F>> for Cartesian3<F> {
    fn from(v: mint::Vector3<F>) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}
impl<F: Scalar> From<Cartesian3<F>> for mint::Vector3<F> {
    fn from(v: Cartesian3<F>) -> Self {
        mint::Vector3 {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}
impl<F: Scalar> From<mint::Point3<F>> for Cartesian3<F> {
    fn from(v: mint::Point3<F>) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}
impl<F: Scalar> From<Cartesian3<F>> for mint::Point3<F> {
    fn from(v: Cartesian3<F>) -> Self {
        mint::Point3 {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

///
/// [F; 3] conversions
///
impl<F: Scalar> From<[F; 3]> for Cartesian3<F> {
    fn from(v: [F; 3]) -> Self {
        Self::new(v[0], v[1], v[2])
    }
}
impl<F: Scalar> From<Cartesian3<F>> for [F; 3] {
    fn from(v: Cartesian3<F>) -> Self {
        [v.x, v.y, v.z]
    }
}

///
/// (F, F, F) conversions
///
impl<F: Scalar> From<(F, F, F)> for Cartesian3<F> {
    fn from(v: (F, F, F)) -> Self {
        Self::new(v.0, v.1, v.2)
    }
}
impl<F: Scalar> From<Cartesian3<F>> for (F, F, F) {
    fn from(v: Cartesian3<F>) -> Self {
        (v.x, v.y, v.z)
    }
}
