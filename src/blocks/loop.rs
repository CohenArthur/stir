//! The Loop block is used to represent ranged and infinite loops.

use super::BasicBlock;

use crate::label::Label;

pub struct Loop<'block> {
    label: Label,
    lo_bound: Option<&'block dyn BasicBlock>,
    hi_bound: Option<&'block dyn BasicBlock>,
    body: Option<&'block dyn BasicBlock>,
}

impl<'block> Loop<'block> {
    /// Create a new Loop block.
    /// Pass None as lo_bound or hi_bound if the loop is an infinite one
    pub fn new(
        lo_bound: Option<&'block dyn BasicBlock>,
        hi_bound: Option<&'block dyn BasicBlock>,
        body: Option<&'block dyn BasicBlock>,
    ) -> Loop<'block> {
        Loop {
            label: Label::new("loop"),
            lo_bound,
            hi_bound,
            body,
        }
    }
}

impl BasicBlock for Loop<'_> {
    fn label(&self) -> &String {
        self.label.name()
    }

    fn interpret(&self) -> bool {
        true // FIXME: Logic: Add logic
    }

    fn output(&self) -> String {
        String::from("loop") // FIXME: Logic: Pretty print
    }
}

impl std::fmt::Debug for Loop<'_> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        dbg!(&self.label);
        dbg!(self.lo_bound);
        dbg!(self.hi_bound);
        dbg!(self.body);

        Ok(())
    }
}
