pub trait RegisterMap<RegType> {
    fn register_offset(&self, reg: u8) -> Option<usize>;
}

impl<T, const N: usize> RegisterMap<T> for [T; N] {
    fn register_offset(&self, reg: u8) -> Option<usize> {
        if self.len() >= reg.into() {
            return None;
        }

        Some(core::mem::size_of::<T>() * (reg as usize))
    }
}
