//! Trait that all `stir::blocks` implement. Allows for code generation and
//! inspection

pub trait BasicBlock {
    /// Allows run-time inspection of the block
    ///
    /// # Example
    ///
    /// ```
    /// use stir::blocks::{Boolean, BasicBlock};
    ///
    /// let b = Boolean::new(false);
    ///
    /// b.debug(); // Also usable in the `fry` interpreter
    /// ```
    fn debug(&self);

    /// Transforms the block into its corresponding STIR representation
    // FIXME: Add example and better doc
    fn output(&self) -> String;
}
