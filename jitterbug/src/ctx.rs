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
pub struct ExecutionContext<Backend: Compiler = PlatformDefaultBackend> {
    backend: RefCell<Backend>,
}

impl<Backend: Compiler> ExecutionContext<Backend> {
    pub fn compile<'ctx, 'state: 'ctx, State: RegisterMap + 'state>(
        &'ctx self,
        translation_unit: Box<TranslationUnit>,
    ) -> Result<CompiledTranslationUnit<State, Backend>, String> {
        let exec = self.compile_unit(&translation_unit)?;

        Ok(CompiledTranslationUnit {
            context: self,
            translation_unit,
            executable: Rc::downgrade(&exec),
        })
    }

    fn compile_unit<'state, State: RegisterMap + 'state>(
        &self,
        unit: &Box<TranslationUnit>,
    ) -> Result<Rc<dyn Executable<State> + 'state>, String> {
        self.backend.borrow_mut().compile_unit(unit)
    }
}

pub struct CompiledTranslationUnit<'ctx, 'state, State: RegisterMap, Backend: Compiler> {
    context: &'ctx ExecutionContext<Backend>,
    translation_unit: Box<TranslationUnit>,
    executable: Weak<dyn Executable<State> + 'state>,
}

impl<'ctx, 'state, State: RegisterMap + 'state, Backend: Compiler>
    CompiledTranslationUnit<'ctx, 'state, State, Backend>
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
