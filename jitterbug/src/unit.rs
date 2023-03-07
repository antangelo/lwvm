use crate::block::{BasicBlock, InstructionStream};
use crate::ir::types::BlockLabel;
use std::collections::BTreeMap;

#[derive(Default, Clone)]
pub struct TranslationUnit {
    pub(crate) labels: BTreeMap<BlockLabel, usize>,
    pub(crate) blocks: Vec<BasicBlock>,
    pub(crate) entrypoint: Option<usize>,
}

impl TranslationUnit {
    pub fn add_basic_block(&mut self, label: BlockLabel, block: BasicBlock) -> Result<(), String> {
        if !block.validate() {
            return Err(String::from(
                "Block is not terminated (All basic blocks must end with a branch or exit)",
            ));
        }

        self.labels.insert(label, self.blocks.len());
        self.blocks.push(block);

        Ok(())
    }

    pub fn set_entry(&mut self, label: BlockLabel) -> Result<(), String> {
        let entry_idx = self
            .labels
            .get(&label)
            .ok_or(format!("No such block {} to make entrypiont", label))?;
        self.entrypoint = Some(*entry_idx);

        Ok(())
    }
}
