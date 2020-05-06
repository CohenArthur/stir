//! Trait that all `stir::blocks` implement. Allows for code generation and
//! inspection

pub trait BasicBlock: std::fmt::Debug {
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
    fn debug(&self) {
        dbg!(self);
    }

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

    /// If the block is critical or if it can safely be parallelized
    ///
    /// # Example
    ///
    /// ```
    /// use stir::blocks::{Boolean, Critical, BasicBlock};
    ///
    /// let non_crit_b = Boolean::new(false);
    ///
    /// let critical_boolean_block = Boolean::new(true);
    /// let critical_b = Critical::new(&critical_boolean_block);
    ///
    /// assert!(critical_b.is_critical());
    /// assert!(!non_crit_b.is_critical());
    /// ```
    fn is_critical(&self) -> bool {
        false
    }
}
