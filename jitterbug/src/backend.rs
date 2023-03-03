use super::context::TranslationUnit;

trait Backend {
    fn compile<T>(&self, unit: &mut TranslationUnit<T>);
}
