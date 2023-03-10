pub mod backend;
pub mod block;
pub mod ctx;
pub mod interpret;
pub mod unit;

mod ir;

pub use ir::types::{IntImmed, LValue};

#[cfg(test)]
mod tests {
    use crate::ctx::ExecutionContext;

    #[test]
    fn it_works() {
        use super::ir::types::{IntImmed, LValue};

        let mut block = super::block::BasicBlock::builder();
        block.add(
            LValue::Register(0),
            IntImmed::I16(-3 as i16 as u16),
            IntImmed::I16(-4 as i16 as u16),
            true,
        );

        let block = block.finish_exit(10);

        let mut unit = super::unit::TranslationUnit::default();
        unit.add_basic_block(String::from("main"), block).unwrap();
        unit.set_entry(String::from("main")).unwrap();

        let ctx: ExecutionContext<super::interpret::InterpreterBackend> =
            ExecutionContext::default();
        let mut tb = ctx.compile(Box::new(unit)).unwrap();

        let mut state = [0u64; 10];

        unsafe {
            tb.execute(&mut state);
        }

        assert_eq!(state[0], -7 as i16 as u64);
    }
}
