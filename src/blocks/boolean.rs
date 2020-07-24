//! This block represents a boolean block. It is either `false` or `true`.
//! It is simply a wrapper around the `bool` type in Rust

use super::{BasicBlock, Primitive};

use crate::label::Label;

/// Wrapper struct around a `bool`
#[derive(Debug)]
pub struct Boolean {
    /// Label of the boolean
    label: Label,

    /// Actual value of the Boolean
    value: bool,
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

impl Primitive for Boolean {
    type ValueType = bool;

    fn get(&self) -> Self::ValueType {
        self.value
    }

    fn set(&mut self, value: Self::ValueType) {
        self.value = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let b = Boolean::new(false);

        assert!(!b.get());
    }

    #[test]
    fn test_mut() {
        let mut b = Boolean::new(false);

        assert_eq!(b.get(), false);

        b.set(true);

        assert!(b.get());
    }

    fn test_value() {
        let b_t = Boolean::new(true);
        let b_f = Boolean::new(false);

        assert!(b_t.get());
        assert!(!b_f.get());
    }
}
