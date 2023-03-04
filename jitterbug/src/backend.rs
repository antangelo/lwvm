use crate::{reg::RegisterMap, unit::TranslationUnit};
use std::rc::Rc;

pub trait Compiler<RegType, State: RegisterMap<RegType>> {
    fn compile_unit(
        &mut self,
        unit: &TranslationUnit,
    ) -> Result<Rc<dyn Executable<RegType, State>>, String>;
}

pub trait Executable<RegType, State: RegisterMap<RegType>> {
    unsafe fn execute(&self, state: &mut State);
}

#[derive(Default)]
pub struct PlatformDefaultBackend {}

impl<RegType, State: RegisterMap<RegType>> Compiler<RegType, State> for PlatformDefaultBackend {
    fn compile_unit(
        &mut self,
        unit: &TranslationUnit,
    ) -> Result<Rc<dyn Executable<RegType, State>>, String> {
        Err(String::from("No platform backend available"))
    }
}
