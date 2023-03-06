use crate::{ir::reg::RegisterMap, unit::TranslationUnit};
use std::rc::Rc;

pub trait Compiler<State: RegisterMap> {
    fn compile_unit(&mut self, unit: &TranslationUnit)
        -> Result<Rc<dyn Executable<State>>, String>;
}

pub trait Executable<State: RegisterMap> {
    unsafe fn execute(&self, state: &mut State);
}

#[derive(Default)]
pub struct PlatformDefaultBackend {}

impl<State: RegisterMap> Compiler<State> for PlatformDefaultBackend {
    fn compile_unit(
        &mut self,
        unit: &TranslationUnit,
    ) -> Result<Rc<dyn Executable<State>>, String> {
        Err(String::from("No platform backend available"))
    }
}
