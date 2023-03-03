use crate::types::{BlockLabel, Comparator, IntImmed, IntType, LValue, RValue};
use crate::ops::Operation;

trait InstructionStream {
    fn to_vec(&self) -> &Vec<Operation>;
}

#[derive(Debug, Default)]
pub struct UnsafeBasicBlock {
    ops: Vec<Operation>,
}

impl InstructionStream for UnsafeBasicBlock {
    fn to_vec(&self) -> &Vec<Operation> {
        &self.ops
    }
}

#[derive(Debug, Default)]
pub struct BasicBlock {
    ops: Vec<Operation>,
}

impl InstructionStream for BasicBlock {
    fn to_vec(&self) -> &Vec<Operation> {
        &self.ops
    }
}

macro_rules! op_lv1_rv1 {
    ($name:ident, $op:ident) => {
        pub fn $name(
            &mut self,
            dest: (impl Into<LValue> + Clone),
            arg1: (impl Into<RValue<IntImmed>> + Clone),
        ) {
            self.ops.push(Operation::$op(
                Into::<LValue>::into(dest),
                Into::<RValue<IntImmed>>::into(arg1),
            ));
        }
    };
}

macro_rules! op_lv1_rv1_ty {
    ($name:ident, $op:ident) => {
        pub fn $name(
            &mut self,
            dest: (impl Into<LValue> + Clone),
            arg1: (impl Into<RValue<IntImmed>> + Clone),
            ty: IntType,
        ) {
            self.ops.push(Operation::$op(
                Into::<LValue>::into(dest),
                Into::<RValue<IntImmed>>::into(arg1),
                ty,
            ));
        }
    };
}

macro_rules! op_lv1_rv2 {
    ($name:ident, $op:ident) => {
        pub fn $name(
            &mut self,
            dest: (impl Into<LValue> + Clone),
            arg1: (impl Into<RValue<IntImmed>> + Clone),
            arg2: (impl Into<RValue<IntImmed>> + Clone),
        ) {
            self.ops.push(Operation::$op(
                Into::<LValue>::into(dest),
                Into::<RValue<IntImmed>>::into(arg1),
                Into::<RValue<IntImmed>>::into(arg2),
            ));
        }
    };
}

macro_rules! op_lv0_rv2 {
    ($name:ident, $op:ident) => {
        pub fn $name(
            &mut self,
            arg1: (impl Into<RValue<IntImmed>> + Clone),
            arg2: (impl Into<RValue<IntImmed>> + Clone),
        ) {
            self.ops.push(Operation::$op(
                Into::<RValue<IntImmed>>::into(arg1),
                Into::<RValue<IntImmed>>::into(arg2),
            ));
        }
    };
}

macro_rules! op_lv1_rv2_signed {
    ($name:ident, $op:ident) => {
        pub fn $name(
            &mut self,
            dest: (impl Into<LValue> + Clone),
            arg1: (impl Into<RValue<IntImmed>> + Clone),
            arg2: (impl Into<RValue<IntImmed>> + Clone),
            signed: bool,
        ) {
            self.ops.push(Operation::$op(
                Into::<LValue>::into(dest),
                Into::<RValue<IntImmed>>::into(arg1),
                Into::<RValue<IntImmed>>::into(arg2),
                signed,
            ));
        }
    };
}

macro_rules! define_safe_ops {
    () => {
        op_lv1_rv2_signed!(add, Add);
        op_lv1_rv2_signed!(sub, Sub);

        op_lv1_rv2_signed!(mult, Mult);
        op_lv1_rv2_signed!(div, Div);
        op_lv1_rv2_signed!(rem, Rem);

        op_lv1_rv2!(shift_left, LShift);
        op_lv1_rv2_signed!(shift_right, RShift);

        op_lv1_rv1_ty!(sign_extend, SignExtend);
        op_lv1_rv1_ty!(zero_extend, ZeroExtend);

        op_lv1_rv2!(and, And);
        op_lv1_rv2!(or, Or);
        op_lv1_rv2!(xor, Xor);
        op_lv1_rv1!(not, Not);

        op_lv1_rv1!(guest_mem_read, GuestReadMem);
        op_lv0_rv2!(guest_mem_write, GuestWriteMem);

        pub fn int_cmp(
            &mut self,
            dest: (impl Into<LValue> + Clone),
            cmp: Comparator,
            arg1: (impl Into<RValue<IntImmed>> + Clone),
            arg2: (impl Into<RValue<IntImmed>> + Clone),
        ) {
            self.ops.push(Operation::ICmp(
                Into::<LValue>::into(dest),
                cmp,
                Into::<RValue<IntImmed>>::into(arg1),
                Into::<RValue<IntImmed>>::into(arg2),
            ));
        }

        pub fn select(
            &mut self,
            cond: (impl Into<RValue<IntImmed>> + Clone),
            dest: (impl Into<LValue> + Clone),
            arg1: (impl Into<RValue<IntImmed>> + Clone),
            arg2: (impl Into<RValue<IntImmed>> + Clone),
        ) {
            self.ops.push(Operation::Select(
                Into::<RValue<IntImmed>>::into(cond),
                Into::<LValue>::into(dest),
                Into::<RValue<IntImmed>>::into(arg1),
                Into::<RValue<IntImmed>>::into(arg2),
            ));
        }

        pub fn branch(
            &mut self,
            cond: (impl Into<RValue<IntImmed>> + Clone),
            label_taken: BlockLabel,
            label_not_taken: BlockLabel,
        ) {
            self.ops.push(Operation::Branch(
                Into::<RValue<IntImmed>>::into(cond),
                label_taken,
                label_not_taken,
            ));
        }

        pub fn exit(&mut self, code: u8) {
            self.ops.push(Operation::Exit(code));
        }
    };
}

impl BasicBlock {
    define_safe_ops!();
}

impl UnsafeBasicBlock {
    define_safe_ops!();

    pub unsafe fn host_mem_read(
        &mut self,
        dest: (impl Into<LValue> + Clone),
        arg1: (impl Into<RValue<IntImmed>> + Clone),
    ) {
        self.ops.push(Operation::HostReadMem(
            Into::<LValue>::into(dest),
            Into::<RValue<IntImmed>>::into(arg1),
        ));
    }

    pub unsafe fn host_mem_write(
        &mut self,
        arg1: (impl Into<RValue<IntImmed>> + Clone),
        arg2: (impl Into<RValue<IntImmed>> + Clone),
    ) {
        self.ops.push(Operation::HostWriteMem(
            Into::<RValue<IntImmed>>::into(arg1),
            Into::<RValue<IntImmed>>::into(arg2),
        ));
    }

    // TODO
    //pub unsafe fn fn_call(&mut self, ptr: (impl Into<RValue<IntImmed>> + Clone)) {}
}
