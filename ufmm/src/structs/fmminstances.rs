use super::Float;
use super::vectors::cartesian3::Cartesian3;

pub mod directsum;
pub trait UserStorage<F: Float> {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn read_position(&self, idx: usize) -> Cartesian3<F>;
    fn read_velocity(&self, idx: usize) -> Cartesian3<F>;
    fn write_acceleration(&mut self, idx: usize);
}

pub trait FMMInstance<F: Float> {
    fn compute(user_storage: &mut impl UserStorage<F>);
}
