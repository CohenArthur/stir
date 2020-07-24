//! Trait that all primitive types implement, such as integers, strings, floats...

pub trait Primitive: super::BasicBlock {
    /// Type of the value contained in the primitive
    type ValueType;

    /// Return the value contained in the block
    ///
    /// # Example
    ///
    /// ```
    /// use stir::blocks::{Primitive, Boolean}
    ///
    /// let b = Boolean::new(true);
    ///
    /// assert!(b.get());
    /// ```
    fn get(&self) -> Self::ValueType;

    /// Set the value contained in the block
    ///
    /// # Example
    ///
    /// ```
    /// use stir::blocks::{Primitive, Boolean}
    ///
    /// let b = Boolean::new(true);
    ///
    /// b.set(false);
    ///
    /// assert!(!b.get());
    /// ```
    fn set(&mut self, value: Self::ValueType);
}
