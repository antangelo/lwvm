pub mod backend;
pub mod block;
pub mod context;

mod types;
mod ops;

pub use types::{IntImmed, LValue};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::types::{LValue, IntImmed};
        let mut block = super::block::BasicBlock::default();
        block.add(LValue::Register(0), IntImmed::I16(3), IntImmed::I16(4), true);
    }
}
