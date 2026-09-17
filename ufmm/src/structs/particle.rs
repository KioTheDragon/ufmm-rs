/*#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Particle<F: Float, U> {
    pub pos: Vector3<F>,
    pub vel: Vector3<F>,
    pub acc: Vector3<F>,
    pub mass: F,
    pub charge: F,
    pub data: U,
}

impl<F: Float, U> Particle<F, U> {
    pub fn new(
        pos: impl Into<Vector3<F>>,
        vel: impl Into<Vector3<F>>,
        mass: F,
        charge: F,
        data: U,
    ) -> Self {
        Particle {
            pos: pos.into(),
            vel: vel.into(),
            acc: Vector3::default(),
            mass,
            charge,
            data,
        }
    }
}*/
