use std::f64::consts::PI;
use std::iter::Sum;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use super::cartesian3::Cartesian3;

use super::Scalar;
use super::Vector3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Spherical3<F: Scalar> {
    pub r: F,
    pub theta: F,
    pub phi: F,
}

// r -      length
//          [0, +inf)
//
// theta -  polar angle from positive z-axis
//          [0, Pi]
//
// phi -    azimuthal angle in xy-plane
//          from positive x-axis
//          counterclockwise
//          [-Pi, Pi]

///
/// define struct Spherical3
///
impl<F: Scalar> Spherical3<F> {
    pub fn new(r: F, theta: F, phi: F) -> Self {
        let (theta, phi) = Self::normalize_angles(theta, phi);
        if r >= F::zero() {
            Self { r, theta, phi }
        } else {
            -Self { r: -r, theta, phi }
        }
    }
}

impl<F: Scalar> Default for Spherical3<F> {
    fn default() -> Self {
        Self::zero()
    }
}

///
/// conversion between cartesian3 and spherical3
///
impl<F: Scalar> From<Cartesian3<F>> for Spherical3<F> {
    fn from(value: Cartesian3<F>) -> Self {
        let r = value.length();
        if r == F::zero() {
            return Spherical3 {
                r: F::zero(),
                theta: F::zero(),
                phi: F::zero(),
            };
        }
        let theta = (value.z / r).clamp(-F::one(), F::one()).acos();
        let phi = (value.y).atan2(value.x);
        Self::new(r, theta, phi)
    }
}

impl<F: Scalar> From<Spherical3<F>> for Cartesian3<F> {
    fn from(value: Spherical3<F>) -> Self {
        Cartesian3::new(
            value.r * value.theta.sin() * value.phi.cos(),
            value.r * value.theta.sin() * value.phi.sin(),
            value.r * value.theta.cos(),
        )
    }
}

///
/// Help function for distance() and distance_squared()
///
impl<F: Scalar> Spherical3<F> {
    fn angular_haversine(&self, rhs: &Self) -> F {
        let sin_dtheta_half = ((self.theta - rhs.theta) / (F::one() + F::one())).sin();
        let sin_dphi_half = ((self.phi - rhs.phi) / (F::one() + F::one())).sin();
        (sin_dtheta_half * sin_dtheta_half
            + self.theta.sin() * rhs.theta.sin() * sin_dphi_half * sin_dphi_half)
            .clamp(F::zero(), F::one())
    }
}
///
/// Help function for normalize angles
///
impl<F: Scalar> Spherical3<F> {
    fn normalize_angles(theta: F, phi: F) -> (F, F) {
        let mut theta = theta;
        let mut phi = phi;
        let scalar_pi = F::from(PI).expect("Cant convert PI<f64> to <Scalar>");
        while theta < F::zero() {
            theta += scalar_pi;
        }
        while theta > scalar_pi {
            theta -= scalar_pi;
        }
        while phi < -scalar_pi {
            phi += scalar_pi + scalar_pi
        }
        while phi > scalar_pi {
            phi -= scalar_pi + scalar_pi
        }
        (theta, phi)
    }
}

///
/// basic math operations
///
// vec + vec
impl<F: Scalar> Add for Spherical3<F> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        (Cartesian3::from(self) + Cartesian3::from(rhs)).into()
    }
}

// vec += vec
impl<F: Scalar> AddAssign for Spherical3<F> {
    fn add_assign(&mut self, rhs: Self) {
        *self = (Cartesian3::from(*self) + Cartesian3::from(rhs)).into();
    }
}

// vec - vec
impl<F: Scalar> Sub for Spherical3<F> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        (Cartesian3::from(self) - Cartesian3::from(rhs)).into()
    }
}

// vec -= vec
impl<F: Scalar> SubAssign for Spherical3<F> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = (Cartesian3::from(*self) - Cartesian3::from(rhs)).into();
    }
}

// -vec
impl<F: Scalar> Neg for Spherical3<F> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        if self.r.is_zero() {
            return Self::zero();
        }
        let scalar_pi = F::from(PI).expect("Cant convert PI<f64> to <Scalar>");
        let (theta, phi) = Self::normalize_angles(scalar_pi - self.theta, self.phi + scalar_pi);
        Self::new(self.r, theta, phi)
    }
}

// vec * scalar
impl<F: Scalar> Mul<F> for Spherical3<F> {
    type Output = Self;
    fn mul(self, rhs: F) -> Self::Output {
        if rhs.is_zero() {
            Self::zero()
        } else if rhs < F::zero() {
            -Self::new(self.r * -rhs, self.theta, self.phi)
        } else {
            Self::new(self.r * rhs, self.theta, self.phi)
        }
    }
}

// vec *= scalar
impl<F: Scalar> MulAssign<F> for Spherical3<F> {
    fn mul_assign(&mut self, rhs: F) {
        if rhs.is_zero() {
            *self = Self::zero()
        } else if rhs < F::zero() {
            self.r *= -rhs;
            *self = -*self;
        } else {
            self.r *= rhs
        }
    }
}

// vec / scalar
impl<F: Scalar> Div<F> for Spherical3<F> {
    type Output = Self;
    fn div(self, rhs: F) -> Self::Output {
        if rhs.is_infinite() {
            Self::zero()
        } else if rhs < F::zero() {
            -Self::new(self.r / -rhs, self.theta, self.phi)
        } else {
            Self::new(self.r / rhs, self.theta, self.phi)
        }
    }
}

// vec /= scalar
impl<F: Scalar> DivAssign<F> for Spherical3<F> {
    fn div_assign(&mut self, rhs: F) {
        if rhs.is_infinite() {
            *self = Self::zero();
        } else if rhs < F::zero() {
            self.r /= -rhs;
            *self = -*self;
        } else {
            self.r /= rhs
        }
    }
}

impl<F: Scalar> Vector3<F> for Spherical3<F> {
    fn zero() -> Self {
        Self::new(F::zero(), F::zero(), F::zero())
    }

    fn dot(self, rhs: Self) -> F {
        let cos = self.theta.cos() * rhs.theta.cos()
            + self.theta.sin() * rhs.theta.sin() * (self.phi - rhs.phi).cos();
        cos * self.r * rhs.r
    }

    fn cross(self, rhs: Self) -> Self {
        (Cartesian3::from(self).cross(Cartesian3::from(rhs))).into()
    }

    fn length_squared(&self) -> F {
        self.r * self.r
    }

    fn length(&self) -> F {
        self.r
    }

    fn normalize(&mut self) {
        if !self.r.is_zero() {
            self.r = F::one();
        }
    }

    fn normalized(&self) -> Self {
        if self.r.is_zero() {
            Self::zero()
        } else {
            Self::new(F::one(), self.theta, self.phi)
        }
    }

    fn distance(&self, rhs: &Self) -> F {
        (self.r - rhs.r)
            .hypot((F::one() + F::one()) * (self.r * rhs.r * self.angular_haversine(rhs)).sqrt())
    }

    fn distance_squared(&self, rhs: &Self) -> F {
        (self.r - rhs.r) * (self.r - rhs.r)
            + (F::one() + F::one() + F::one() + F::one())
                * self.r
                * rhs.r
                * self.angular_haversine(rhs)
    }

    fn angle_between(&self, rhs: &Self) -> F {
        if self.r.is_zero() || rhs.r.is_zero() {
            return F::zero();
        }
        (self.theta.cos() * rhs.theta.cos()
            + self.theta.sin() * rhs.theta.sin() * (self.phi - rhs.phi).cos())
        .clamp(-F::one(), F::one())
        .acos()
    }
}

// vec[i]
impl<F: Scalar> Index<usize> for Spherical3<F> {
    type Output = F;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.r,
            1 => &self.theta,
            2 => &self.phi,
            _ => panic!("index out of bounds: the len is 3 but the index is {index}"),
        }
    }
}

// vec[i] = . . .
impl<F: Scalar> IndexMut<usize> for Spherical3<F> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.r,
            1 => &mut self.theta,
            2 => &mut self.phi,
            _ => panic!("index out of bounds: the len is 3 but the index is {index}"),
        }
    }
}

// sum of [vec]
impl<F: Scalar> Sum for Spherical3<F> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |acc, v| acc + v)
    }
}
