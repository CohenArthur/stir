//! A Function block is a block containing other blocks. It basically
//! contains a sequence of other blocks to execute one by one.

use super::BasicBlock;

use crate::label::Label;

use std::vec::Vec;

#[derive(Debug)]
pub struct Function<'block> {
    label: Label,
    args: Option<&'block Vec<&'block dyn BasicBlock>>,
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
    /// let mut vec: Vec<&dyn BasicBlock> = vec!(&f_block, &t_block);
    ///
    /// // Create a function with no arguments and no return value
    /// let function_block = Function::new(None, &vec);
    ///
    /// assert_eq!(function_block.interpret(), false);
    /// ```
    pub fn new(
        args: Option<&'block Vec<&'block dyn BasicBlock>>,
        stmts: &'block Vec<&'block dyn BasicBlock>,
    ) -> Function<'block> {
        Function {
            label: Label::new("function"),
            args,
            stmts,
            retval: None,
        }
    }

    pub fn set_retval(&mut self, retval: &'block dyn BasicBlock) {
        self.retval = Some(retval);
    }

    pub fn get_arg(&self, idx: usize) -> Option<&'block dyn BasicBlock> {
        match self.args {
            Some(args) => Some(args[idx]),
            None => None,
        }
    }
}

impl BasicBlock for Function<'_> {
    fn label(&self) -> &String {
        self.label.name()
    }

    fn interpret(&self) -> bool {
        for statement in self.stmts.iter() {
            statement.interpret();
        }

        match self.retval {
            Some(val) => val.interpret(),
            None => false,
        }
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

        let f = Function::new(None, &vec);

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

        let f = Function::new(None, &vec);

        assert_eq!(f.interpret(), false);
    }

    #[test]
    fn test_true_stmt() {
        let t = Boolean::new(true);

        let mut vec: Vec<&dyn BasicBlock> = Vec::new();
        vec.push(&t);

        let f = Function::new(None, &vec);

        assert!(!f.interpret());
    }

    #[test]
    fn test_args() {
        let arg0 = Boolean::new(false);
        let arg1 = Boolean::new(true);
        let arg2 = Boolean::new(true);

        let args: Vec<&dyn BasicBlock> = vec![&arg0, &arg1, &arg2];

        let no_body = vec![] as Vec<&dyn BasicBlock>;
        let f = Function::new(Some(&args), &no_body);

        assert_eq!(f.get_arg(0).unwrap().label(), arg0.label());
        assert_eq!(f.get_arg(1).unwrap().label(), arg1.label());
        assert_eq!(f.get_arg(2).unwrap().label(), arg2.label());
    }

    #[test]
    fn test_retval() {
        let true_retval = Boolean::new(true);

        let no_body = vec![] as Vec<&dyn BasicBlock>;

        let mut f = Function::new(None, &no_body);
        f.set_retval(&true_retval);

        assert!(f.interpret());
    }
}
