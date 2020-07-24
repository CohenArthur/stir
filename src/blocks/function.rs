//! A Function block is a block containing other blocks. It basically
//! contains a sequence of other blocks to execute one by one.

use super::BasicBlock;

use crate::label::Label;

use std::vec::Vec;

#[derive(Debug)]
pub struct Function<'block> {
    label: Label,
    args: &'block Vec<&'block dyn BasicBlock>,
    stmts: &'block Vec<&'block dyn BasicBlock>,
    retval: Option<&'block dyn BasicBlock>,
}

impl<'block> Function<'block> {
    /// Create a new function block from a vector of blocks
    ///
    /// # Example
    ///
    /// ```
    /// use stir::blocks::{Boolean, Function, BasicBlock};
    ///
    /// let f_block = Boolean::new(false);
    /// let t_block = Boolean::new(true);
    ///
    /// let mut vec: Vec<&dyn BasicBlock> = Vec::new();
    ///
    /// vec.push(&f_block);
    /// vec.push(&t_block);
    ///
    /// let function_block = Function::new(&vec);
    ///
    /// assert_eq!(function_block.interpret(), false);
    /// ```
    pub fn new(args: &'block Vec<&'block dyn BasicBlock>, stmts: &'block Vec<&'block dyn BasicBlock>) -> Function<'block> {
        Function {
            label: Label::new("function"),
            args,
            stmts,
            retval: None,
        }
    }

    pub fn with_retval(&mut self, retval: &'block dyn BasicBlock) -> &'block Function {
        self.retval = Some(retval);

        self
    }
}

impl BasicBlock for Function<'_> {
    fn label(&self) -> &String {
        self.label.name()
    }

    fn interpret(&self) -> bool {
        let mut res = true;

        for statement in self.stmts.iter() {
            // FIXME: Add actual logic ? Or is this what we want ?
            res = if statement.interpret() { res } else { false };
        }

        res
    }

    fn output(&self) -> String {
        String::from("function") // FIXME: Add logic
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks::{Boolean, IfElse};

    #[test]
    fn test_single_stmt() {
        let b = Boolean::new(false);

        let mut vec: Vec<&dyn BasicBlock> = Vec::new();
        vec.push(&b);

        let f = Function::new(&vec);

        assert_eq!(f.interpret(), false);
    }

    #[test]
    fn test_multi_stmt() {
        let c = Boolean::new(true);
        let t = Boolean::new(false);
        let f = Boolean::new(true);
        let ie = IfElse::new(&c, &t, Some(&f));

        let other_stmt = Boolean::new(true);

        let mut vec: Vec<&dyn BasicBlock> = Vec::new();

        vec.push(&ie);
        vec.push(&other_stmt);

        let f = Function::new(&vec);

        assert_eq!(f.interpret(), false);
    }

    #[test]
    fn test_true_stmt() {
        let t = Boolean::new(true);

        let mut vec: Vec<&dyn BasicBlock> = Vec::new();
        vec.push(&t);

        let f = Function::new(&vec);

        assert!(f.interpret());
    }
}
