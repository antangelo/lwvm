use crate::{
    backend::{Compiler, Executable},
    ir::{
        ops::Operation,
        reg::{Register, RegisterMap, RegisterType},
        types::{BlockLabel, RValue, ZippedIntImmed},
    },
    unit::TranslationUnit,
    IntImmed, LValue,
};
use std::rc::Rc;

#[derive(Default)]
pub struct InterpreterBackend {}

enum ExitAction {
    Exit(u8),
    BranchTo(usize),
}

pub struct InterpreterExecutable {
    unit: TranslationUnit,
    regs: Vec<Register>,
}

impl InterpreterExecutable {
    fn rv_to_immed<State: RegisterMap>(&self, state: &State, rv: &RValue<IntImmed>) -> IntImmed {
        match rv {
            RValue::Immediate(i) => *i,
            RValue::LValue(lv) => match lv {
                LValue::Register(r) => unsafe { self.regs[*r as usize].read(state) },
                LValue::Scratch(_) => unimplemented!(),
            },
        }
    }

    fn op_add<State: RegisterMap>(
        &self,
        dest: &LValue,
        arg1: &RValue<IntImmed>,
        arg2: &RValue<IntImmed>,
        signed: bool,
        state: &mut State,
    ) {
        let arg1 = self.rv_to_immed(state, arg1);
        let arg2 = self.rv_to_immed(state, arg2);
        let args = IntImmed::upcast_zip(&arg1, &arg2, signed);
        let value = match args {
            ZippedIntImmed::Bool(v1, v2) => IntImmed::I8(if v1 && v2 { 2 } else if v1 || v2 { 1 } else { 0 }),
            ZippedIntImmed::I8(v1, v2) => IntImmed::I8(v1.wrapping_add(v2)),
            ZippedIntImmed::I16(v1, v2) => IntImmed::I16(v1.wrapping_add(v2)),
            ZippedIntImmed::I32(v1, v2) => IntImmed::I32(v1.wrapping_add(v2)),
            ZippedIntImmed::I64(v1, v2) => IntImmed::I64(v1.wrapping_add(v2)),
        };

        let value = if signed {
            value.to_i64() as u64
        } else {
            value.to_u64()
        };

        match dest {
            LValue::Register(r) => unsafe {
                self.regs[*r as usize].write(value, state);
            },
            LValue::Scratch(_) => unimplemented!(),
        }
    }

    fn op_sub<State: RegisterMap>(
        &self,
        dest: &LValue,
        arg1: &RValue<IntImmed>,
        arg2: &RValue<IntImmed>,
        signed: bool,
        state: &mut State,
    ) {
        let arg1 = self.rv_to_immed(state, arg1);
        let arg2 = self.rv_to_immed(state, arg2);
        let args = IntImmed::upcast_zip(&arg1, &arg2, signed);
        let value = match args {
            ZippedIntImmed::Bool(v1, v2) => IntImmed::I8(if v1 == v2 { 0 } else if v1 { 1 } else { 0xff }),
            ZippedIntImmed::I8(v1, v2) => IntImmed::I8(v1.wrapping_sub(v2)),
            ZippedIntImmed::I16(v1, v2) => IntImmed::I16(v1.wrapping_sub(v2)),
            ZippedIntImmed::I32(v1, v2) => IntImmed::I32(v1.wrapping_sub(v2)),
            ZippedIntImmed::I64(v1, v2) => IntImmed::I64(v1.wrapping_sub(v2)),
        };

        let value = if signed {
            value.to_i64() as u64
        } else {
            value.to_u64()
        };

        match dest {
            LValue::Register(r) => unsafe {
                self.regs[*r as usize].write(value, state);
            },
            LValue::Scratch(_) => unimplemented!(),
        }
    }

    fn op_branch<State: RegisterMap>(
        &self,
        cond: &RValue<IntImmed>,
        taken: &BlockLabel,
        not_taken: &BlockLabel,
        state: &mut State,
    ) -> ExitAction {
        let value = self.rv_to_immed(state, cond).to_u64();
        let branch_sel = if value == 0 { not_taken } else { taken };

        let idx = self.unit.labels.get(branch_sel).unwrap();
        ExitAction::BranchTo(*idx)
    }

    fn execute_block<State: RegisterMap>(
        &self,
        ops: &Vec<Operation>,
        state: &mut State,
    ) -> ExitAction {
        for op in ops {
            match op {
                Operation::Add(dest, arg1, arg2, signed) => {
                    self.op_add(dest, arg1, arg2, *signed, state)
                },
                Operation::Sub(dest, arg1, arg2, signed) => {
                    self.op_sub(dest, arg1, arg2, *signed, state)
                }
                Operation::Exit(code) => return ExitAction::Exit(*code),
                Operation::Branch(cond, taken, not_taken) => {
                    return self.op_branch(cond, taken, not_taken, state)
                }
                o => panic!("Unimplemented operation {:?}", o),
            }
        }

        panic!("Non-terminating block");
    }
}

impl Compiler for InterpreterBackend {
    fn compile_unit<'a, State: RegisterMap + 'a>(
        &mut self,
        unit: &TranslationUnit,
    ) -> Result<std::rc::Rc<dyn Executable<State> + 'a>, String> {
        Ok(Rc::new(InterpreterExecutable {
            unit: unit.clone(),
            regs: State::register_offsets(),
        }))
    }
}

impl<State: RegisterMap> Executable<State> for InterpreterExecutable {
    unsafe fn execute(&self, state: &mut State) {
        let mut idx = self.unit.entrypoint.unwrap();
        loop {
            let block = &(self.unit.blocks[idx]);
            let exit_action = self.execute_block(&block.ops, state);
            match exit_action {
                ExitAction::Exit(_) => return,
                ExitAction::BranchTo(branch_idx) => idx = branch_idx,
            }
        }
    }
}
