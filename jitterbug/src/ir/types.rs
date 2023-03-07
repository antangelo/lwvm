pub type BlockLabel = String;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum IntType {
    Bool,
    I8,
    I16,
    I32,
    I64,
}

macro_rules! impl_from_type {
    ($ty:ident, $name:ident) => {
        pub fn $name(&self, v: $ty) -> IntImmed {
            match self {
                Self::Bool => IntImmed::Bool(v != 0),
                Self::I8 => IntImmed::I8(v as u8),
                Self::I16 => IntImmed::I16(v as u16),
                Self::I32 => IntImmed::I32(v as u32),
                Self::I64 => IntImmed::I64(v as u64),
            }
        }
    };
}

macro_rules! impl_from_type_signed {
    ($ty:ident, $name:ident) => {
        pub fn $name(&self, v: $ty) -> IntImmed {
            match self {
                Self::Bool => IntImmed::Bool(v != 0),
                Self::I8 => IntImmed::I8(v as i8 as u8),
                Self::I16 => IntImmed::I16(v as i16 as u16),
                Self::I32 => IntImmed::I32(v as i32 as u32),
                Self::I64 => IntImmed::I64(v as i64 as u64),
            }
        }
    };
}

impl IntType {
    impl_from_type!(u8, from_u8);
    impl_from_type!(u16, from_u16);
    impl_from_type!(u32, from_u32);
    impl_from_type!(u64, from_u64);

    impl_from_type_signed!(i8, from_u8_signed);
    impl_from_type_signed!(i16, from_u16_signed);
    impl_from_type_signed!(i32, from_u32_signed);
    impl_from_type_signed!(i64, from_u64_signed);
}

#[derive(Debug, Clone, Copy)]
pub enum IntImmed {
    Bool(bool),
    I8(u8),
    I16(u16),
    I32(u32),
    I64(u64),
}

impl From<u32> for IntImmed {
    fn from(value: u32) -> Self {
        Self::I32(value)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ZippedIntImmed {
    Bool(bool, bool),
    I8(u8, u8),
    I16(u16, u16),
    I32(u32, u32),
    I64(u64, u64),
}

macro_rules! impl_maybe_type {
    ($ty:ty, $br:ident, $name:ident) => {
        pub fn $name(&self) -> Option<$ty> {
            match self {
                Self::$br(v) => Some(*v),
                _ => None,
            }
        }
    };
}

impl IntImmed {
    impl_maybe_type!(bool, Bool, maybe_bool);
    impl_maybe_type!(u8, I8, maybe_u8);
    impl_maybe_type!(u16, I16, maybe_u16);
    impl_maybe_type!(u32, I32, maybe_u32);
    impl_maybe_type!(u64, I64, maybe_u64);

    pub fn size(&self) -> u8 {
        match self {
            Self::Bool(_) => 1,
            Self::I8(_) => 8,
            Self::I16(_) => 16,
            Self::I32(_) => 32,
            Self::I64(_) => 64,
        }
    }

    pub fn to_u64(&self) -> u64 {
        match *self {
            Self::Bool(b) => b as u64,
            Self::I8(i) => i as u64,
            Self::I16(i) => i as u64,
            Self::I32(i) => i as u64,
            Self::I64(i) => i,
        }
    }

    pub fn to_i64(&self) -> i64 {
        match *self {
            Self::Bool(b) => b as i64,
            Self::I8(i) => i as i8 as i64,
            Self::I16(i) => i as i16 as i64,
            Self::I32(i) => i as i32 as i64,
            Self::I64(i) => i as i64,
        }
    }

    pub fn cast(&self, ty: IntType, signed: bool) -> Self {
        if signed {
            match self {
                Self::Bool(b) => ty.from_u8_signed(if *b { 1 } else { 0 }),
                Self::I8(i) => ty.from_u8_signed(*i as i8),
                Self::I16(i) => ty.from_u16_signed(*i as i16),
                Self::I32(i) => ty.from_u32_signed(*i as i32),
                Self::I64(i) => ty.from_u64_signed(*i as i64),
            }
        } else {
            match self {
                Self::Bool(b) => ty.from_u8(if *b { 1 } else { 0 }),
                Self::I8(i) => ty.from_u8(*i),
                Self::I16(i) => ty.from_u16(*i),
                Self::I32(i) => ty.from_u32(*i),
                Self::I64(i) => ty.from_u64(*i),
            }
        }
    }

    /// Casts the smaller of first, second up to the larger type
    /// Returns a pair of IntImmed values such that both are
    /// the same internal type.
    pub fn upcast(first: &Self, second: &Self, signed: bool) -> (Self, Self) {
        let first_size = first.size();
        let second_size = second.size();

        if first_size == second_size {
            return (*first, *second);
        }

        let (larger, smaller) = if first_size > second_size {
            (first, second)
        } else {
            (second, first)
        };

        (*larger, smaller.cast(larger.get_type(), signed))
    }

    pub fn upcast_zip(first: &Self, second: &Self, signed: bool) -> ZippedIntImmed {
        let (larger, smaller) = Self::upcast(first, second, signed);

        match larger {
            Self::Bool(b) => ZippedIntImmed::Bool(b, smaller.maybe_bool().unwrap()),
            Self::I8(v) => ZippedIntImmed::I8(v, smaller.maybe_u8().unwrap()),
            Self::I16(v) => ZippedIntImmed::I16(v, smaller.maybe_u16().unwrap()),
            Self::I32(v) => ZippedIntImmed::I32(v, smaller.maybe_u32().unwrap()),
            Self::I64(v) => ZippedIntImmed::I64(v, smaller.maybe_u64().unwrap()),
        }
    }

    pub fn get_type(&self) -> IntType {
        match self {
            Self::Bool(_) => IntType::Bool,
            Self::I8(_) => IntType::I8,
            Self::I16(_) => IntType::I16,
            Self::I32(_) => IntType::I32,
            Self::I64(_) => IntType::I64,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RValue<T> {
    LValue(LValue),
    Immediate(T),
}

#[derive(Debug, Clone, Copy)]
pub enum LValue {
    Register(u8),
    Scratch(u8),
}

impl<T> From<LValue> for RValue<T> {
    fn from(value: LValue) -> Self {
        Self::LValue(value)
    }
}

impl From<IntImmed> for RValue<IntImmed> {
    fn from(value: IntImmed) -> Self {
        Self::Immediate(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Comparator {
    EQ,
    NEQ,
    SLT,
    SGT,
    ULT,
    UGT,
}

#[derive(Debug)]
pub enum Condition {}

// #[derive(Debug, Clone, Eq)]
// pub struct StructType {
//     fields: Vec<Type>,
// }

// impl PartialEq for StructType {
//     fn eq(&self, other: &Self) -> bool {
//         self.fields.iter().eq(other.fields.iter())
//     }
// }

// #[derive(Debug)]
// pub struct StructTypeImmed {
//     ty: StructType,
//     field_vals: Vec<TypeImmed>,
// }

// impl StructTypeImmed {
//     fn validate_fields(&self) -> bool {
//         if self.ty.fields.len() != self.field_vals.len() {
//             return false;
//         }

//         self.field_vals
//             .iter()
//             .map(|f| f.get_type())
//             .zip(self.ty.fields.iter())
//             .fold(true, |a, b| a && b.0 == *b.1)
//     }

//     fn get_type(&self) -> &StructType {
//         &self.ty
//     }
// }

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum Type {
//     Integer(IntType),
//     Struct(StructType),
// }

// #[derive(Debug)]
// pub enum TypeImmed {
//     Integer(IntImmed),
//     Struct(StructTypeImmed),
// }

// impl TypeImmed {
//     fn get_type(&self) -> Type {
//         match self {
//             Self::Integer(i) => Type::Integer(i.get_type()),
//             Self::Struct(s) => Type::Struct(s.get_type().clone()),
//         }
//     }
// }
