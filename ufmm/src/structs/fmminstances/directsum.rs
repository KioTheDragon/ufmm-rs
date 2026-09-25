use super::FMMInstance;
use super::Float;

pub struct DirectSum {}

impl<F: Float> FMMInstance<F> for DirectSum {
    fn compute(user_storage: &mut impl super::UserStorage<F>) {}
}
