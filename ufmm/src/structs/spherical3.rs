use super::Scalar;
use super::Vector3;
use super::Vector3Interop;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Spherical3<F: Scalar> {
    pub r: F,
    pub theta: F,
    pub phi: F,
}

impl<F: Scalar> Spherical3<F> {
    pub fn new(r: F, theta: F, phi: F) -> Self {
        Self { r, theta, phi }
    }
}
