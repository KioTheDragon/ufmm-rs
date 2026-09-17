use super::cartesian3::Cartesian3;

use super::Scalar;
use super::Vector3;
use super::Vector3Interop;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Spherical3<F: Scalar> {
    pub r: F,
    pub theta: F,
    pub phi: F,
}

// r -      length
// theta -  polar angle from positive z-axis
// phi -    azimuthal angle in xy-plane
//          from positive x-axis
//          counterclockwise

///
/// define struct Cartesian3
///
impl<F: Scalar> Spherical3<F> {
    pub fn new(r: F, theta: F, phi: F) -> Self {
        Self { r, theta, phi }
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
        let theta = (value.z / r).acos();
        let phi = (value.y).atan2(value.x);
        Spherical3::new(r, theta, phi)
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

impl<F: Scalar> Vector3<F> for Spherical3<F> {}
