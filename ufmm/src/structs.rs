use std::{
    fmt::Debug,
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

use num_traits::Float;

pub mod cartesian3;
pub mod particle;
pub mod spherical3;

pub trait Scalar:
    Float         // уже включает Copy, PartialOrd, Neg, Add/Sub/Mul/Div, Zero, One, NumCast
    + Debug
    + AddAssign    // F += F
    + SubAssign    // F -= F
    + MulAssign    // F *= F
    + DivAssign    // F /= F
    + Send
    + Sync
    + 'static
{ }
impl<T> Scalar for T where
    T: Float + Debug + AddAssign + SubAssign + MulAssign + DivAssign + Send + Sync + 'static
{
}
pub trait Vector3<F: Scalar>:
    Sized
    + Copy
    + PartialEq
    + Debug
    + Default
    // vec & vec math
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Neg<Output = Self>
    // vec & scalar math
    + Mul<F, Output = Self>
    + MulAssign<F>
    + Div<F, Output = Self>
    + DivAssign<F>
    // vec[i] access
    + Index<usize, Output = F>
    + IndexMut<usize>
    // sum of [vec]
    + Sum<Self>
{
    fn zero() -> Self;
    /// vec * vec
    fn dot(self, rhs: Self) -> F;

    /// vec x vec
    fn cross(self, rhs: Self) -> Self;

    /// |vec|^2
    fn length_squared(&self) -> F;

    /// |vec|
    fn length(&self) -> F;

    /// make |vec| = 1;
    /// if |vec| = 0 then dont touch anything
    fn normalize(&mut self);

    /// copy with |vec| = 1
    /// if |vec| = 0 then return vec with (0, 0, 0)
    fn normalized(&self) -> Self;

    /// |vec1 - vec2|
    fn distance(&self, rhs: &Self) -> F;

    /// |vec1 - vec2|^2
    fn distance_squared(&self, rhs: &Self) -> F;

    /// angle(vec1, vec2) in [[0, π]]
    fn angle_between(&self, rhs: &Self) -> F;
}

/// Converse with all others 3-dimensional vectors
pub trait Vector3Interop<F: Scalar>:
    Vector3<F>
    + MintVector3Convertible<F>
    + MintPoint3Convertible<F>
    + Array3Convertible<F>
    + Tuple3Convertible<F>
{
}

/// Converse with `mint::Vector3<F>`.
pub trait MintVector3Convertible<F: Scalar>:
    From<mint::Vector3<F>> + Into<mint::Vector3<F>>
{
}
impl<T, F: Scalar> MintVector3Convertible<F> for T where
    T: From<mint::Vector3<F>> + Into<mint::Vector3<F>>
{
}
/// Converse with `mint::Point3<F>`.
pub trait MintPoint3Convertible<F: Scalar>: From<mint::Point3<F>> + Into<mint::Point3<F>> {}
impl<T, F: Scalar> MintPoint3Convertible<F> for T where
    T: From<mint::Point3<F>> + Into<mint::Point3<F>>
{
}
/// Converse with `[F; 3]`.
pub trait Array3Convertible<F: Scalar>: From<[F; 3]> + Into<[F; 3]> {}
impl<T, F: Scalar> Array3Convertible<F> for T where T: From<[F; 3]> + Into<[F; 3]> {}
/// Converse with `(F, F, F)`.
pub trait Tuple3Convertible<F: Scalar>: From<(F, F, F)> + Into<(F, F, F)> {}
impl<T, F: Scalar> Tuple3Convertible<F> for T where T: From<(F, F, F)> + Into<(F, F, F)> {}
