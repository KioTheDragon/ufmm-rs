pub mod morton1xu64;
use super::Unsigned;
use std::hash::Hash;
pub trait MortonBits<U: Unsigned>: Copy + Eq + Ord + Hash + Send + Sync + 'static {
    /// max depth, defined by
    const MAX_DEPTH: u8;
    // make morton from quantum coords
    fn new(qx: U, qy: U, qz: U) -> Self;
    /// get octant of specified level (0 = root).
    fn get_octant(&self, level: u8) -> u8;
}
