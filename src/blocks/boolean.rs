//! This block represents a boolean block. It is either `false` or `true`.
//! It is simply a wrapper around the `bool` type in Rust

use super::BasicBlock;

use crate::label::Label;

/// Wrapper struct around a `bool`
#[derive(Debug)]
pub struct Boolean {
    /// Actual value of the Boolean
    value: bool,

    /// Label of the boolean
    label: Label,
}

impl Boolean {
    /// Allocate a new Boolean
    ///
    /// # Example
    ///
    /// ```
    /// use stir::blocks::Boolean;
    ///
    /// let b = Boolean::new(false);
    /// ```
    pub fn new(value: bool) -> Boolean {
        Boolean {
            value,
            label: Label::new("bool"),
        }
    }

    /// Return the value contained in a Boolean
    ///
    /// # Example
    ///
    /// ```
    /// use stir::blocks::Boolean;
    ///
    /// let b = Boolean::new(true);
    ///
    /// assert!(b.value());
    /// ```
    pub fn value(&self) -> bool {
        self.value
    }

    /// Set a Boolean's value
    ///
    /// # Example
    ///
    /// ```
    /// use stir::blocks::Boolean;
    ///
    /// let mut b = Boolean::new(false);
    /// b.set(true);
    ///
    /// assert!(b.value());
    /// ```
    pub fn set(&mut self, value: bool) {
        self.value = value;
    }
}

impl BasicBlock for Boolean {
    fn label(&self) -> &String {
        self.label.name()
    }

    fn debug(&self) {
        dbg!(self);
    }

    fn output(&self) -> String {
        // FIXME: syntax: Use actual syntax
        if self.value {
            String::from("true")
        } else {
            String::from("false")
        }
    }

    fn interpret(&self) -> bool {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let b = Boolean::new(false);

        assert!(!b.value());
    }

    #[test]
    fn test_mut() {
        let mut b = Boolean::new(false);

        assert_eq!(b.value(), false);

        b.set(true);

        assert!(b.value());
    }
}
