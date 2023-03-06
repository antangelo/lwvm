use crate::{
    backend::{Compiler, Executable, PlatformDefaultBackend},
    ir::reg::RegisterMap,
    unit::TranslationUnit,
};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Default)]
pub struct ExecutionContext<State: RegisterMap, Backend: Compiler<State> = PlatformDefaultBackend> {
    backend: RefCell<Backend>,

    phantom_state: core::marker::PhantomData<State>,
}

impl<State: RegisterMap, Backend: Compiler<State>> ExecutionContext<State, Backend> {
    pub fn compile(
        &self,
        translation_unit: Box<TranslationUnit>,
    ) -> Result<CompiledTranslationUnit<State, Backend>, String> {
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
    ) -> Result<Rc<dyn Executable<State>>, String> {
        self.backend.borrow_mut().compile_unit(unit)
    }
}

pub struct CompiledTranslationUnit<'ctx, State: RegisterMap, Backend: Compiler<State>> {
    context: &'ctx ExecutionContext<State, Backend>,
    translation_unit: Box<TranslationUnit>,
    executable: Weak<dyn Executable<State>>,
}

impl<'ctx, State: RegisterMap, Backend: Compiler<State>>
    CompiledTranslationUnit<'ctx, State, Backend>
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
