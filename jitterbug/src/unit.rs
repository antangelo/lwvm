use crate::block::{BasicBlock, InstructionStream, UnsafeBasicBlock};
use crate::types::BlockLabel;
use std::collections::BTreeMap;

#[derive(Default)]
pub struct TranslationUnit {
    blocks: BTreeMap<BlockLabel, BasicBlock>,
    entrypoint: Option<BlockLabel>,
}

impl TranslationUnit {
    pub fn add_basic_block(&mut self, label: BlockLabel, block: BasicBlock) -> Result<(), String> {
        if !block.validate() {
            return Err(String::from(
                "Block is not terminated (All basic blocks must end with a branch or exit)",
            ));
        }

        self.blocks.insert(label, block);

        Ok(())
    }

    pub fn set_entry(&mut self, label: BlockLabel) -> Result<(), String> {
        if !self.blocks.contains_key(&label) {
            return Err(format!("No such block {} to make entrypiont", label));
        }

        self.entrypoint = Some(label);

        Ok(())
    }
}

pub(crate) enum MaybeUnsafeBlock {
    Safe(BasicBlock),
    Unsafe(UnsafeBasicBlock),
}

#[derive(Default)]
pub struct UnsafeTranslationUnit {
    blocks: BTreeMap<BlockLabel, MaybeUnsafeBlock>,
    entrypoint: Option<BlockLabel>,
}

impl UnsafeTranslationUnit {
    pub fn add_basic_block(&mut self, label: BlockLabel, block: BasicBlock) -> Result<(), String> {
        if !block.validate() {
            return Err(String::from(
                "Block is not terminated (All basic blocks must end with a branch or exit)",
            ));
        }

        self.blocks.insert(label, MaybeUnsafeBlock::Safe(block));

        Ok(())
    }

    pub unsafe fn add_unsafe_basic_block(
        &mut self,
        label: BlockLabel,
        block: UnsafeBasicBlock,
    ) -> Result<(), String> {
        if !block.validate() {
            return Err(String::from(
                "Block is not terminated (All basic blocks must end with a branch or exit)",
            ));
        }

        self.blocks.insert(label, MaybeUnsafeBlock::Unsafe(block));

        Ok(())
    }

    pub fn set_entry(&mut self, label: BlockLabel) -> Result<(), String> {
        if !self.blocks.contains_key(&label) {
            return Err(format!("No such block {} to make entrypiont", label));
        }

        self.entrypoint = Some(label);

        Ok(())
    }
}