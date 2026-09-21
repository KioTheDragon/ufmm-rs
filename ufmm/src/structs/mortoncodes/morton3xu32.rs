use super::MortonCode;

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
pub struct Morton3xU32 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl MortonCode for Morton3xU32 {}
impl Ord for Morton3xU32 {
    fn cmp(&self, _other: &Self) -> std::cmp::Ordering {
        todo!()
    }
}
impl PartialOrd for Morton3xU32 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
