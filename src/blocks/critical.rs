//! A Critical block is a block that shall not be multithreaded.

use super::BasicBlock;

pub struct Critical<'block> {
    block: &'block dyn BasicBlock,
}

impl <'block> Critical<'block> {
    /// Create a new Critical block and wrap it around another block
    /// When noticing a critical block in the syntax, create said block and
    /// then wrap it in a Critical block.
    pub fn new(block: &'block dyn BasicBlock) -> Critical<'block> {
        Critical {
            block
        }
    }
}

impl BasicBlock for Critical<'_> {
    fn label(&self) -> &String {
        self.block.label()
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
}
