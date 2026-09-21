use std::hash::Hash;

pub mod morton3xu32;

pub trait MortonCode: Copy + Eq + Ord + Hash {}
