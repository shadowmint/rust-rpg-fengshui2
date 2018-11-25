pub struct D6 {
    /// Does a roll of 6 explode to an extra dice value?
    explodes: bool
}

impl D6 {
    /// Return a single unmodified, non-exploding dice value
    pub fn single() -> D6 {
        return D6 {
            explodes: false
        };
    }

    /// Resolve this dice roll
    pub fn resolve(&self) -> usize {
        let mut value = 0;
        while value % 6 == 0 {
            value += rand::random::<u8>();
            if !self.explodes {
                break;
            }
        }
        return value as usize;
    }
}

