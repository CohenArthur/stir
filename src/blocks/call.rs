//! A Call block is a block whose aim is to call a function. It has arguments,
//! as well as a return value. It takes a vector of arguments and can only
//! return one value at a time.

use super::BasicBlock;
use super::Function;

use crate::label::Label;

#[derive(Debug)]
pub struct Call<'block> {
    label: Label,
    function: &'block Function<'block>,
    args: Option<&'block Vec<&'block dyn BasicBlock>>,
}

impl<'block> Call<'block> {
    /// Create a new call block
    ///
    /// # Example
    ///
    /// ```
    /// use stir::blocks::{Boolean, Call, Function, BasicBlock};
    ///
    /// let arg0 = Boolean::new(true);
    /// let args: Vec<&dyn BasicBlock> = vec!(&arg0);
    ///
    /// // A very useful function
    /// let body0 = Boolean::new(false);
    /// let body1 = Boolean::new(false);
    /// let body2 = Boolean::new(true);
    /// let vec: Vec<&dyn BasicBlock> = vec!(&body0, &body1, &body2);
    /// let function = Function::new(None, &vec);
    ///
    /// // Create the calling block with the boolean argument
    /// let call = Call::new(&function, Some(&args));
    ///
    /// assert_eq!(call.interpret(), false);
    /// ```
    pub fn new(
        function: &'block Function,
        args: Option<&'block Vec<&'block dyn BasicBlock>>,
    ) -> Call<'block> {
        Call {
            label: Label::new("call"),
            function,
            args,
        }
    }
}

impl BasicBlock for Call<'_> {
    fn label(&self) -> &String {
        self.label.name()
    }

    fn interpret(&self) -> bool {
        self.function.interpret()
    }

    fn output(&self) -> String {
        format!("CALL {}", self.function.label())
    }
}
