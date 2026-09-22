use super::MortonCode;
use std::cmp::Ordering;

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
pub struct Morton3xU32 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl MortonCode for Morton3xU32 {}
impl Ord for Morton3xU32 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Все различающиеся биты по всем осям.
        let diff = (self.x ^ other.x) | (self.y ^ other.y) | (self.z ^ other.z);
        if diff == 0 {
            return Ordering::Equal;
        }
        let m = 0x8000_0000u32 >> diff.leading_zeros();
        let z = (self.z & m).cmp(&(other.z & m));
        if z != Ordering::Equal {
            return z;
        }
        let y = (self.y & m).cmp(&(other.y & m));
        if y != Ordering::Equal {
            return y;
        }
        (self.x & m).cmp(&(other.x & m))
    }
}
impl PartialOrd for Morton3xU32 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
