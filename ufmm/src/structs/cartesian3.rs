use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

use num_traits::Float;

//
// define
//

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cartesian3<F: Float> {
    pub x: F,
    pub y: F,
    pub z: F,
}

impl<F: Float> Cartesian3<F> {
    pub fn new(x: F, y: F, z: F) -> Self {
        Self { x, y, z }
    }
}

//
// default values
//

impl<F: Float> Default for Cartesian3<F> {
    fn default() -> Self {
        Self::zero()
    }
}
impl<F: Float> Cartesian3<F> {
    // (0, 0, 0)
    pub fn zero() -> Self {
        Self::new(F::zero(), F::zero(), F::zero())
    }
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

//
// operations
//

// vec + vec
impl<F: Float> Add for Cartesian3<F> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

// vec += vec
impl<F: Float> AddAssign for Cartesian3<F> {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

// vec - vec
impl<F: Float> Sub for Cartesian3<F> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

// vec -= vec
impl<F: Float> SubAssign for Cartesian3<F> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
    }
}

// -vec
impl<F: Float> Neg for Cartesian3<F> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

// scalar * vec
macro_rules! impl_scalar_mul {
    ($($t:ty),* $(,)?) => {$(
        impl Mul<Cartesian3<$t>> for $t {
            type Output = Cartesian3<$t>;
            fn mul(self, rhs: Cartesian3<$t>) -> Self::Output {
                Cartesian3::new(rhs.x * self, rhs.y * self, rhs.z * self)
            }
        }
    )*};
}
impl_scalar_mul!(f32, f64);

// vec * scalar
impl<F: Float> Mul<F> for Cartesian3<F> {
    type Output = Self;
    fn mul(self, rhs: F) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

// vec *= scalar
impl<F: Float> MulAssign<F> for Cartesian3<F> {
    fn mul_assign(&mut self, rhs: F) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}

// vec / scalar
impl<F: Float> Div<F> for Cartesian3<F> {
    type Output = Self;
    fn div(self, rhs: F) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

// vec /= scalar
impl<F: Float> DivAssign<F> for Cartesian3<F> {
    fn div_assign(&mut self, rhs: F) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
    }
}

impl<F: Float> Cartesian3<F> {
    // vec * vec
    pub fn dot(self, rhs: Cartesian3<F>) -> F {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    // vec x vec
    pub fn cross(self, rhs: Cartesian3<F>) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
    // return length^2
    pub fn len_squared(&self) -> F {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    // return length
    pub fn len(&self) -> F {
        self.len_squared().sqrt()
    }
    // convert to |vec| = 1
    pub fn normalize(&mut self) {
        let len = self.len();
        if len.is_zero() {
            *self = Self::zero();
        } else {
            *self /= len;
        }
    }
    // return normalized vec
    pub fn normalized(&self) -> Self {
        let len = self.len();
        if len.is_zero() {
            Self::zero()
        } else {
            *self / len
        }
    }
    pub fn distance(&self, rhs: &Self) -> F {
        (*self - *rhs).len()
    }
    pub fn distance_squared(&self, rhs: &Self) -> F {
        (*self - *rhs).len_squared()
    }
    pub fn angle_between(&self, rhs: &Self) -> F {
        let denom = self.len() * rhs.len();
        if denom.is_zero() {
            return F::zero();
        }
        let cos = (self.dot(*rhs) / denom).clamp(-F::one(), F::one());
        cos.acos()
    }
}

// vec[i]
impl<F: Float> Index<usize> for Cartesian3<F> {
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
impl<F: Float> IndexMut<usize> for Cartesian3<F> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of bounds: the len is 3 but the index is {index}"),
        }
    }
}

// sum of [v1, v2, v3]
impl<F: Float> Sum for Cartesian3<F> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |acc, v| acc + v)
    }
}

//
// mint conversions
//

impl<F: Float> From<mint::Vector3<F>> for Cartesian3<F> {
    fn from(v: mint::Vector3<F>) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl<F: Float> From<Cartesian3<F>> for mint::Vector3<F> {
    fn from(v: Cartesian3<F>) -> Self {
        mint::Vector3 {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl<F: Float> From<mint::Point3<F>> for Cartesian3<F> {
    fn from(v: mint::Point3<F>) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl<F: Float> From<Cartesian3<F>> for mint::Point3<F> {
    fn from(v: Cartesian3<F>) -> Self {
        mint::Point3 {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

//
// [F; 3] conversions
//

impl<F: Float> From<[F; 3]> for Cartesian3<F> {
    fn from(v: [F; 3]) -> Self {
        Self::new(v[0], v[1], v[2])
    }
}
impl<F: Float> From<Cartesian3<F>> for [F; 3] {
    fn from(v: Cartesian3<F>) -> Self {
        [v.x, v.y, v.z]
    }
}

//
// (F, F, F) conversions
//

impl<F: Float> From<(F, F, F)> for Cartesian3<F> {
    fn from(v: (F, F, F)) -> Self {
        Self::new(v.0, v.1, v.2)
    }
}

impl<F: Float> From<Cartesian3<F>> for (F, F, F) {
    fn from(v: Cartesian3<F>) -> Self {
        (v.x, v.y, v.z)
    }
}
