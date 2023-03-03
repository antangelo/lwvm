pub type BlockLabel = String;

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

// impl IntImmed {
//     fn get_type(&self) -> IntType {
//         match self {
//             Self::Bool(_) => IntType::Bool,
//             Self::I8(_) => IntType::I8,
//             Self::I16(_) => IntType::I16,
//             Self::I32(_) => IntType::I32,
//             Self::I64(_) => IntType::I64,
//         }
//     }
// }

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum IntType {
    Bool,
    I8,
    I16,
    I32,
    I64,
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
