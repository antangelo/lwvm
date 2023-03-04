pub mod backend;
pub mod block;
pub mod context;
pub mod unit;

mod ops;
mod reg;
mod types;

pub use types::{IntImmed, LValue};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::types::{IntImmed, LValue};

        let mut block = super::block::BasicBlock::default();
        block.add(
            LValue::Register(0),
            IntImmed::I16(3),
            IntImmed::I16(4),
            true,
        );
        block.exit(10);

        let mut unit = super::unit::TranslationUnit::default();
        unit.add_basic_block(String::from("main"), block).unwrap();
        unit.set_entry(String::from("main")).unwrap();

        let ctx = super::context::ExecutionContext::<u64, [u64; 10]>::default();
        let mut tb = ctx.compile(Box::new(unit)).unwrap();

        let mut state = [0u64; 10];

        unsafe {
            tb.execute(&mut state);
        }
    }
}
