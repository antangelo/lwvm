pub enum RegisterType {
    I8,
    I16,
    I32,
    I64,
    // TODO: Support 128 bit integer registers
    // TODO: Support floating point registers
    // TODO: Support vector registers
}

/// Allows a type to be used as a RegisterType within
/// a state struct definition.
/// Safety:
/// - The type must have the same size as the register it maps to
/// - The memory layout of the type should match that of the register type
/// it is mapped to (e.g. mapping f32 to RegisterType::I32 is invalid)
/// - to_reg_type() must return the same register type on every call
unsafe trait AsRegister {
    fn to_reg_type() -> RegisterType;
}

unsafe impl AsRegister for u8 {
    fn to_reg_type() -> RegisterType {
        RegisterType::I8
    }
}

unsafe impl AsRegister for u16 {
    fn to_reg_type() -> RegisterType {
        RegisterType::I16
    }
}

unsafe impl AsRegister for u32 {
    fn to_reg_type() -> RegisterType {
        RegisterType::I32
    }
}

unsafe impl AsRegister for u64 {
    fn to_reg_type() -> RegisterType {
        RegisterType::I64
    }
}

pub trait RegisterMap {
    fn register_offset(&self, reg: u8) -> Option<(usize, RegisterType)>;
}

impl<T: AsRegister, const N: usize> RegisterMap for [T; N] {
    fn register_offset(&self, reg: u8) -> Option<(usize, RegisterType)> {
        if self.len() >= reg.into() {
            return None;
        }

        Some((core::mem::size_of::<T>() * (reg as usize), T::to_reg_type()))
    }
}
