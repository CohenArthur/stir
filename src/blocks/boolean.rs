//! This block represents a boolean block. It is either `false` or `true`.
//! It is simply a wrapper around the `bool` type in Rust

use super::BasicBlock;

/// Wrapper struct around a `bool`
#[derive(Debug)]
pub struct Boolean {
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
            value
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boolean_test() {
        let mut b = Boolean::new(false);

        assert_eq!(b.value(), false);

        b.set(true);

        assert!(b.value());
    }
}
