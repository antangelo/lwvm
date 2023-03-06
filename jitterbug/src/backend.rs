use crate::{ir::reg::RegisterMap, unit::TranslationUnit};
use std::rc::Rc;

pub trait Compiler {
    fn compile_unit<'a, State: RegisterMap + 'a>(
        &mut self,
        unit: &TranslationUnit,
    ) -> Result<Rc<dyn Executable<State> + 'a>, String>;
}

pub trait Executable<State: RegisterMap> {
    unsafe fn execute(&self, state: &mut State);
}

#[derive(Default)]
pub struct PlatformDefaultBackend {}

impl Compiler for PlatformDefaultBackend {
    fn compile_unit<'a, State: RegisterMap + 'a>(
        &mut self,
        unit: &TranslationUnit,
    ) -> Result<Rc<dyn Executable<State> + 'a>, String> {
        Err(String::from("No platform backend available"))
    }
}
