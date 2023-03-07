use crate::IntImmed;

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
    fn register_offsets() -> Vec<Register>;
}

impl<T: AsRegister, const N: usize> RegisterMap for [T; N] {
    fn register_offsets() -> Vec<Register> {
        (0..N)
            .map(|i| Register {
                offset: core::mem::size_of::<T>() * i,
                ty: T::to_reg_type(),
            })
            .collect()
    }
}

pub struct Register {
    pub(crate) offset: usize,
    pub(crate) ty: RegisterType,
}

impl Register {
    pub(crate) unsafe fn read<State: RegisterMap>(&self, state: &State) -> IntImmed {
        let state = (state as *const State as *const u8).add(self.offset);
        match self.ty {
            RegisterType::I8 => IntImmed::I8(*state),
            RegisterType::I16 => IntImmed::I16(*(state as *const u16)),
            RegisterType::I32 => IntImmed::I32(*(state as *const u32)),
            RegisterType::I64 => IntImmed::I64(*(state as *const u64)),
        }
    }

    pub(crate) fn trunc_to_type(&self, immed: IntImmed) -> IntImmed {
        let value = immed.to_u64();
        match self.ty {
            RegisterType::I8 => IntImmed::I8(value as u8),
            RegisterType::I16 => IntImmed::I16(value as u16),
            RegisterType::I32 => IntImmed::I32(value as u32),
            RegisterType::I64 => IntImmed::I64(value as u64),
        }
    }

    pub(crate) unsafe fn write<State: RegisterMap>(&self, value: u64, state: &mut State) {
        let reg = (state as *mut State as *mut u8).add(self.offset);
        match self.ty {
            RegisterType::I8 => *reg = value as u8,
            RegisterType::I16 => *(reg as *mut u16) = value as u16,
            RegisterType::I32 => *(reg as *mut u32) = value as u32,
            RegisterType::I64 => *(reg as *mut u64) = value,
        }
    }
}
