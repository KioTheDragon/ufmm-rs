pub mod directsum;
pub trait UserStorage {}

pub trait FMMInstance {
    fn compute(user_storage: &mut impl UserStorage) {}
}
