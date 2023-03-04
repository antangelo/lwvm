use crate::{
    backend::{Compiler, Executable, PlatformDefaultBackend},
    reg::RegisterMap,
    unit::TranslationUnit,
};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Default)]
pub struct ExecutionContext<
    RegType,
    State: RegisterMap<RegType>,
    Backend: Compiler<RegType, State> = PlatformDefaultBackend,
> {
    backend: RefCell<Backend>,

    phantom_reg_type: core::marker::PhantomData<RegType>,
    phantom_state: core::marker::PhantomData<State>,
}

impl<RegType, State: RegisterMap<RegType>, Backend: Compiler<RegType, State>>
    ExecutionContext<RegType, State, Backend>
{
    pub fn compile(
        &self,
        translation_unit: Box<TranslationUnit>,
    ) -> Result<CompiledTranslationUnit<RegType, State, Backend>, String> {
        let exec = self.compile_unit(&translation_unit)?;

        Ok(CompiledTranslationUnit {
            context: self,
            translation_unit,
            executable: Rc::downgrade(&exec),
        })
    }

    fn compile_unit(
        &self,
        unit: &Box<TranslationUnit>,
    ) -> Result<Rc<dyn Executable<RegType, State>>, String> {
        self.backend.borrow_mut().compile_unit(unit)
    }
}

pub struct CompiledTranslationUnit<
    'ctx,
    RegType,
    State: RegisterMap<RegType>,
    Backend: Compiler<RegType, State>,
> {
    context: &'ctx ExecutionContext<RegType, State, Backend>,
    translation_unit: Box<TranslationUnit>,
    executable: Weak<dyn Executable<RegType, State>>,
}

impl<'ctx, RegType, State: RegisterMap<RegType>, Backend: Compiler<RegType, State>>
    CompiledTranslationUnit<'ctx, RegType, State, Backend>
{
    pub unsafe fn execute(&mut self, state: &mut State) {
        if let Some(exec) = self.executable.upgrade() {
            unsafe {
                exec.execute(state);
            }
        } else {
            let exec = self.context.compile_unit(&self.translation_unit).unwrap();
            self.executable = Rc::downgrade(&exec);

            unsafe {
                exec.execute(state);
            }
        }
    }
}
