//! Trait that all `stir::blocks` implement. Allows for code generation and
//! inspection

pub trait BasicBlock {
    /// Return the unique label of the block
    fn label(&self) -> &String;

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

    /// Interpret and execute a block
    ///
    /// # Example
    ///
    /// ```
    /// use stir::blocks::{Boolean, BasicBlock};
    ///
    /// let b = Boolean::new(true);
    ///
    /// assert!(b.interpret());
    /// ```
    // FIXME: Logic: Return Result ?
    fn interpret(&self) -> bool;
}

impl std::fmt::Debug for dyn BasicBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ICE" // FIXME: Add actual message
        )
    }
}
