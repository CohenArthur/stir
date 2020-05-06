//! A Critical block is a block that shall not be multithreaded. Critical blocks
//! wrap around any kind of block.

use super::BasicBlock;

use crate::label::Label;

pub struct Critical<'block> {
    label: Label,
    block: &'block dyn BasicBlock,
}

impl<'block> Critical<'block> {
    /// Create a new Critical block and wrap it around another block.
    /// When noticing a critical block in the syntax, create said block and
    /// then wrap it in a Critical block.
    pub fn new(block: &'block dyn BasicBlock) -> Critical<'block> {
        Critical {
            label: Label::new("critical"),
            block,
        }
    }
}

impl BasicBlock for Critical<'_> {
    fn label(&self) -> &String {
        self.label.name()
    }

    fn interpret(&self) -> bool {
        self.block.interpret()
    }

    fn debug(&self) {
        self.block.debug()
    }

    fn output(&self) -> String {
        self.block.output()
    }

    fn is_critical(&self) -> bool {
        true
    }
}

impl std::fmt::Debug for Critical<'_> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        dbg!(self.block);
        Ok(())
    }
}
