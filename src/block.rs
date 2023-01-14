use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct InstructionStream {
    block: Rc<RefCell<IStreamInternal>>,
}

impl Default for InstructionStream {
    fn default() -> Self {
        Self {
            block: Rc::default(),
        }
    }
}

impl InstructionStream {
    pub fn push(&mut self, op: super::ir::Operation) {
        self.block.borrow_mut().push(op);
    }
}

#[derive(Debug)]
struct IStreamInternal {
    ops: Vec<super::ir::Operation>,
}

impl Default for IStreamInternal {
    fn default() -> Self {
        Self{
            ops: vec![],
        }
    }
}

impl IStreamInternal {
    fn push(&mut self, op: super::ir::Operation) {
        self.ops.push(op);
    }
}
