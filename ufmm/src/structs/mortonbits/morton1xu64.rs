use super::MortonBits;
use super::Unsigned;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Morton1xU64 {
    pub bits: u64,
}

impl Morton1xU64 {}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "bmi2")]
unsafe fn interleave3_bmi2(qx: u64, qy: u64, qz: u64) -> u64 {
    use core::arch::x86_64::_pdep_u64;

    const X_MASK: u64 = 0x1249249249249249;
    const Y_MASK: u64 = 0x2492492492492492;
    const Z_MASK: u64 = 0x4924924924924924;
    const COORD_MASK: u64 = (1 << 21) - 1;

    let x = _pdep_u64(qx & COORD_MASK, X_MASK);
    let y = _pdep_u64(qy & COORD_MASK, Y_MASK);
    let z = _pdep_u64(qz & COORD_MASK, Z_MASK);
    x | y | z
}

#[inline(always)]
fn split_by_3(a: u64) -> u64 {
    let mut x = a & 0x1f_ffff;
    x = (x | (x << 32)) & 0x1f00000000ffff;
    x = (x | (x << 16)) & 0x1f0000ff0000ff;
    x = (x | (x << 8)) & 0x100f00f00f00f00f;
    x = (x | (x << 4)) & 0x10c30c30c30c30c3;
    x = (x | (x << 2)) & 0x1249249249249249;
    x
}

#[inline]
fn interleave3_fallback(qx: u64, qy: u64, qz: u64) -> u64 {
    split_by_3(qx) | (split_by_3(qy) << 1) | (split_by_3(qz) << 2)
}

#[inline]
fn interleave3(qx: u64, qy: u64, qz: u64) -> u64 {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("bmi2") {
            // SAFETY: checked
            return unsafe { interleave3_bmi2(qx, qy, qz) };
        }
    }
    interleave3_fallback(qx, qy, qz)
}
impl<U: Unsigned> MortonBits<U> for Morton1xU64 {
    const MAX_DEPTH: u8 = 21;

    fn new(qx: U, qy: U, qz: U) -> Self {
        let qx = qx.to_u64().expect("qx does not fit into u64");
        let qy = qy.to_u64().expect("qy does not fit into u64");
        let qz = qz.to_u64().expect("qz does not fit into u64");
        Morton1xU64 {
            bits: interleave3(qx, qy, qz),
        }
    }
    fn get_octant(&self, _level: u8) -> u8 {
        todo!()
    }
}
