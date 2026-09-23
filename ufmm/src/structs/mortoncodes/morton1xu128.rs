pub struct Morton1xU128 {
    pub code: u128,
}

impl Morton1xU128 {
    pub fn new(q_x: u64, q_y: u64, q_z: u64) -> Self {
        const MASK: u64 = (1u64 << 42) - 1;
        let q_x = q_x & MASK;
        let q_y = q_y & MASK;
        let q_z = q_z & MASK;

        let mut code: u128 = 0;
        for i in 0..42u32 {
            let x = ((q_x >> i) & 1) as u128;
            let y = ((q_y >> i) & 1) as u128;
            let z = ((q_z >> i) & 1) as u128;
            // XYZ порядок: X -> бит 3i, Y -> 3i+1, Z -> 3i+2
            code |= x << (3 * i);
            code |= y << (3 * i + 1);
            code |= z << (3 * i + 2);
        }

        // Сдвигаем влево на 2, чтобы младшие 2 бита всегда были 0.
        // Итог занимает биты [2..=127] (126 значащих бит).
        Self { code: code << 2 }
    }
}
