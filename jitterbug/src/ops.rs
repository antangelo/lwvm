use crate::types::{LValue, RValue, IntImmed, IntType, Comparator, BlockLabel};

#[derive(Debug, Clone)]
pub(crate) enum Operation {
    Add(LValue, RValue<IntImmed>, RValue<IntImmed>, bool),
    Sub(LValue, RValue<IntImmed>, RValue<IntImmed>, bool),

    Mult(LValue, RValue<IntImmed>, RValue<IntImmed>, bool),
    Div(LValue, RValue<IntImmed>, RValue<IntImmed>, bool),
    Rem(LValue, RValue<IntImmed>, RValue<IntImmed>, bool),

    LShift(LValue, RValue<IntImmed>, RValue<IntImmed>),
    RShift(LValue, RValue<IntImmed>, RValue<IntImmed>, bool),
    SignExtend(LValue, RValue<IntImmed>, IntType),
    ZeroExtend(LValue, RValue<IntImmed>, IntType),

    And(LValue, RValue<IntImmed>, RValue<IntImmed>),
    Or(LValue, RValue<IntImmed>, RValue<IntImmed>),
    Xor(LValue, RValue<IntImmed>, RValue<IntImmed>),
    Not(LValue, RValue<IntImmed>),

    HostReadMem(LValue, RValue<IntImmed>),
    HostWriteMem(RValue<IntImmed>, RValue<IntImmed>),
    //FnCall(RValue<IntImmed>, Vec<RValue<IntImmed>>),

    GuestReadMem(LValue, RValue<IntImmed>),
    GuestWriteMem(RValue<IntImmed>, RValue<IntImmed>),

    ICmp(LValue, Comparator, RValue<IntImmed>, RValue<IntImmed>),
    Select(RValue<IntImmed>, LValue, RValue<IntImmed>, RValue<IntImmed>),

    Branch(RValue<IntImmed>, BlockLabel, BlockLabel),
    Exit(u8),
}
