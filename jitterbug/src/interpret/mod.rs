use crate::{
    backend::{Compiler, Executable},
    ir::{reg::{RegisterMap, RegisterType, Register}, types::{BlockLabel, RValue}, ops::Operation},
    unit::TranslationUnit, IntImmed, LValue,
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
fn rvalue_to_u64<State: RegisterMap>(&self, state: &State, rv: &RValue<IntImmed>) -> u64 {
    match rv {
            RValue::Immediate(i) => {
                match i {
                    IntImmed::Bool(b) => *b as u64,
                    IntImmed::I8(i) => *i as u64,
                    IntImmed::I16(i) => *i as u64,
                    IntImmed::I32(i) => *i as u64,
                    IntImmed::I64(i) => *i as u64,
                }
            },
            RValue::LValue(lv) => {
                match lv {
                    LValue::Register(r) => {
                        unsafe {
                            let Register {offset, ty} = &self.regs[*r as usize];
                            let reg = (state as *const State as *const u8).add(*offset);
                            match ty {
                                RegisterType::I8 => *reg as u64,
                                RegisterType::I16 => *(reg as *const u16) as u64,
                                RegisterType::I32 => *(reg as *const u32) as u64,
                                RegisterType::I64 => *(reg as *const u64),
                            }
                        }
                    },
                    LValue::Scratch(_) => unimplemented!(),
                }
            },
        }
}

    fn op_add<State: RegisterMap>(&self, dest: &LValue, arg1: &RValue<IntImmed>, arg2: &RValue<IntImmed>, signed: bool, state: &mut State) {
        let arg1 = self.rvalue_to_u64(state, arg1);
        let arg2 = self.rvalue_to_u64(state, arg2);

        let value = if signed {
            (arg1 as i64).wrapping_add(arg2 as i64) as u64
        } else {
            arg1 + arg2
        };

        match dest {
            LValue::Register(r) => {
                unsafe {
                    let Register { offset, ty } = &self.regs[*r as usize];
                    let reg = (state as *mut State as *mut u8).add(*offset);
                    match ty {
                        RegisterType::I8 => *reg = value as u8,
                        RegisterType::I16 => *(reg as *mut u16) = value as u16,
                        RegisterType::I32 => *(reg as *mut u32) = value as u32,
                        RegisterType::I64 => *(reg as *mut u64) = value,
                    }
                }
            },
            LValue::Scratch(_) => unimplemented!(),
        }
    }

    fn op_branch<State: RegisterMap>(&self, cond: &RValue<IntImmed>, taken: &BlockLabel, not_taken: &BlockLabel, state: &mut State) -> ExitAction {
        let value = self.rvalue_to_u64(state, cond);
        let branch_sel = if value == 0 {
            not_taken
        } else { taken };

        let idx = self.unit.labels.get(branch_sel).unwrap();
        ExitAction::BranchTo(*idx)
    }

    fn execute_block<State: RegisterMap>(&self, ops: &Vec<Operation>, state: &mut State) -> ExitAction {
        for op in ops {
            match op {
                Operation::Add(dest, arg1, arg2, signed) => self.op_add(dest, arg1, arg2, *signed, state),
                Operation::Exit(code) => return ExitAction::Exit(*code),
                Operation::Branch(cond, taken, not_taken) => return self.op_branch(cond, taken, not_taken, state),
                o => panic!("Unimplemented operation {:?}", o),
            }
        };

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
