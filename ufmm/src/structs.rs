pub mod kernels;
pub mod mortonbits;
pub mod translationschemas;
pub mod vectors;

use num_traits::Float as NumFloat;
use num_traits::ToPrimitive;
use num_traits::Unsigned as NumUnsigned;
use std::{
    fmt::Debug,
    ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign},
};

pub trait Float:
    NumFloat + Debug + AddAssign + SubAssign + MulAssign + DivAssign + Send + Sync + 'static
{
}
impl<T> Float for T where
    T: NumFloat + Debug + AddAssign + SubAssign + MulAssign + DivAssign + Send + Sync + 'static
{
}
pub trait Unsigned:
    NumUnsigned
    + ToPrimitive
    + Debug
    + Copy
    + Ord
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
    + Send
    + Sync
    + 'static
{
}

impl<T> Unsigned for T where
    T: NumUnsigned
        + ToPrimitive
        + Debug
        + Copy
        + Ord
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + RemAssign
        + Send
        + Sync
        + 'static
{
}
