use num_traits::Float;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3<F: Float> {
    pub x: F,
    pub y: F,
    pub z: F,
}

impl<F: Float> Default for Vector3<F> {
    fn default() -> Self {
        Self::new(F::zero(), F::zero(), F::zero())
    }
}

impl<F: Float> Vector3<F> {
    pub fn new(x: F, y: F, z: F) -> Self {
        Self { x, y, z }
    }
}

impl<F: Float> From<mint::Vector3<F>> for Vector3<F> {
    fn from(v: mint::Vector3<F>) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl<F: Float> From<Vector3<F>> for mint::Vector3<F> {
    fn from(v: Vector3<F>) -> Self {
        mint::Vector3 {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl<F: Float> From<[F; 3]> for Vector3<F> {
    fn from(v: [F; 3]) -> Self {
        Self::new(v[0], v[1], v[2])
    }
}
impl<F: Float> From<Vector3<F>> for [F; 3] {
    fn from(v: Vector3<F>) -> Self {
        [v.x, v.y, v.z]
    }
}

impl<F: Float> From<(F, F, F)> for Vector3<F> {
    fn from(v: (F, F, F)) -> Self {
        Self::new(v.0, v.1, v.2)
    }
}

impl<F: Float> From<Vector3<F>> for (F, F, F) {
    fn from(v: Vector3<F>) -> Self {
        (v.x, v.y, v.z)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
}
