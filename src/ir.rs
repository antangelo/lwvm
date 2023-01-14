#[derive(Debug)]
pub enum Type {
    Bool(bool),
    I8(u8),
    I16(u16),
    I32(u32),
    I64(u64),
}

#[derive(Debug)]
pub enum RValue {
    LValue(LValue),
    Immediate(Type),
}

#[derive(Debug, Clone, Copy)]
pub enum LValue {
    Register(u8),
    Scratch(u8),
}

impl From<LValue> for RValue {
    fn from(value: LValue) -> Self {
        Self::LValue(value)
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Operation {
    Add(LValue, RValue, RValue),
    Sub(LValue, RValue, RValue),

    UMult(LValue, RValue, RValue),
    SMult(LValue, RValue, RValue),
    UDiv(LValue, RValue, RValue),
    SDiv(LValue, RValue, RValue),
    URem(LValue, RValue, RValue),
    SRem(LValue, RValue, RValue),

    LShift(LValue, RValue, RValue),
    RShift(LValue, RValue, RValue, bool),
    SignExtend(LValue, RValue),
    ZeroExtend(LValue, RValue),

    And(LValue, RValue, RValue),
    Or(LValue, RValue, RValue),
    Xor(LValue, RValue, RValue),
    Not(LValue, RValue),

    HostReadMem(LValue, RValue),
    HostWriteMem(RValue, RValue),

    //FnCall(RValue),

    GuestReadMem(LValue, RValue),
    GuestWriteMem(RValue, RValue),

    ICmp(LValue, Comparator, RValue, RValue),
    BranchIf(RValue, super::block::InstructionStream),
    Select(RValue, LValue, RValue, RValue),

    Exit(u8),
}
