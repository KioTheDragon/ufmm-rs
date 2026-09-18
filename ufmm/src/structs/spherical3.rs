use std::f64::consts::PI;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use super::cartesian3::Cartesian3;

use super::Scalar;
use super::Vector3;

#[derive(Clone, Copy, Debug)]
pub struct Spherical3<F: Scalar> {
    r: F,
    theta: F,
    phi: F,
}

// r -      length
//          [0, +inf)
//          r CAN be +inf, but when it happens, something is probably wrong
//
// theta -  polar angle from positive z-axis
//          [0, Pi]
//
// phi -    azimuthal angle in xy-plane
//          from positive x-axis
//          counterclockwise
//          [-Pi, Pi]

// all of spherical3 with r == 0 are equal to each other

///
/// define struct Spherical3
///
impl<F: Scalar> Spherical3<F> {
    pub fn new(r: F, theta: F, phi: F) -> Self {
        assert!(!r.is_nan(), "r cant be NaN");
        assert!(theta.is_finite(), "theta must be finite");
        assert!(phi.is_finite(), "phi must be finite");
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
/// access to fields with safety
///
impl<F: Scalar> Spherical3<F> {
    pub fn get_r(&self) -> F {
        self.r
    }
    pub fn set_r(&mut self, r: F) -> &mut Self {
        assert!(!r.is_nan(), "r cant be NaN");
        if r.is_zero() {
            *self = Self::zero()
        } else if r < F::zero() {
            *self = -*self;
            self.r = -r;
        } else {
            self.r = r
        }
        self
    }
    pub fn get_theta(&self) -> F {
        self.theta
    }
    pub fn set_theta(&mut self, theta: F) -> &mut Self {
        assert!(theta.is_finite(), "theta must be finite");
        (self.theta, self.phi) = Self::normalize_angles(theta, self.phi);
        self
    }
    pub fn get_phi(&self) -> F {
        self.phi
    }
    pub fn set_phi(&mut self, phi: F) -> &mut Self {
        assert!(phi.is_finite(), "phi must be finite");
        self.phi = Self::normalize_phi(phi);
        self
    }
}

///
/// conversion between cartesian3 and spherical3
///
impl<F: Scalar> From<Cartesian3<F>> for Spherical3<F> {
    fn from(value: Cartesian3<F>) -> Self {
        assert!(
            !value.x.is_nan() && !value.y.is_nan() && !value.z.is_nan(),
            "Trying to convert cartesian3 with NaN inside to spherical3"
        );
        let r = value.length();
        if r == F::zero() {
            return Spherical3::zero();
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
/// Help functions for normalize angles
///
impl<F: Scalar> Spherical3<F> {
    // phi in [-Pi, Pi], so dont need to touch theta
    fn normalize_phi(phi: F) -> F {
        let scalar_pi = F::from(PI).expect("Cant convert PI<f64> to <Scalar>");
        let scalar_2pi = scalar_pi + scalar_pi;
        let mut phi = phi % scalar_2pi;
        if phi < -scalar_pi {
            phi += scalar_2pi;
        } else if phi > scalar_pi {
            phi -= scalar_2pi;
        }
        phi
    }
    // but theta in [0, Pi], and if theta goes in [Pi, 2Pi], we need to rotate vector with phi
    fn normalize_angles(theta: F, phi: F) -> (F, F) {
        let scalar_pi = F::from(PI).expect("Cant convert PI<f64> to <Scalar>");
        let scalar_2pi = scalar_pi + scalar_pi;
        let mut theta = theta % scalar_2pi;
        if theta < F::zero() {
            theta += scalar_2pi;
        }
        let mut phi = Self::normalize_phi(phi);
        if theta > scalar_pi {
            theta = scalar_2pi - theta;
            phi = Self::normalize_phi(phi + scalar_pi);
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
        Self::new(self.r, scalar_pi - self.theta, self.phi + scalar_pi)
    }
}

// vec * scalar
impl<F: Scalar> Mul<F> for Spherical3<F> {
    type Output = Self;
    fn mul(self, rhs: F) -> Self::Output {
        if rhs.is_zero() {
            return Self::zero();
        }
        // any not defined by math or rules operation is banned
        assert!(!(self.r * rhs).is_nan(), "NaN result in math operation");
        *(self.clone().set_r(self.r * rhs))
    }
}

// vec *= scalar
impl<F: Scalar> MulAssign<F> for Spherical3<F> {
    fn mul_assign(&mut self, rhs: F) {
        if rhs.is_zero() {
            *self = Self::zero();
            return;
        }
        // any not defined by math or rules operation is banned
        assert!(!(self.r * rhs).is_nan(), "NaN result in math operation");
        self.set_r(self.r * rhs);
    }
}

// vec / scalar
impl<F: Scalar> Div<F> for Spherical3<F> {
    type Output = Self;
    fn div(self, rhs: F) -> Self::Output {
        if rhs.is_infinite() {
            return Self::zero();
        }
        // any not defined by math or rules operation is banned
        assert!(!rhs.is_zero(), "Division by zero");
        assert!(!(self.r / rhs).is_nan(), "NaN result in math operation");
        *(self.clone().set_r(self.r / rhs))
    }
}

// vec /= scalar
impl<F: Scalar> DivAssign<F> for Spherical3<F> {
    fn div_assign(&mut self, rhs: F) {
        if rhs.is_infinite() {
            *self = Self::zero();
            return;
        }
        // any not defined by math operation is banned
        assert!(!rhs.is_zero(), "Division by zero");
        assert!(!(self.r / rhs).is_nan(), "NaN result in math operation");
        self.set_r(self.r / rhs);
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

// sum of [vec]
impl<F: Scalar> Sum for Spherical3<F> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |acc, v| acc + v)
    }
}

// check vec == vec correctly
impl<F: Scalar> PartialEq for Spherical3<F> {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r
            && (self.r.is_zero() || (self.theta == other.theta && self.phi == other.phi))
    }
}
