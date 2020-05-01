//! This block represents a boolean block. It is either `false` or `true`.
//! It is simply a wrapper around the `bool` type in Rust

/// Wrapper struct around a `bool`
pub struct Boolean {
    /// Actual value of the Boolean
    value: bool,
}

impl Boolean {
    /// Allocate a new Boolean
    pub fn new(value: bool) -> Boolean {
        Boolean {
            value
        }
    }

    /// Return the value contained in a Boolean
    pub fn value(&self) -> bool {
        self.value
    }

    /// Set a Boolean's value
    pub fn set(&mut self, value: bool) {
        self.value = value;
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