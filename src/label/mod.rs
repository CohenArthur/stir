use lazy_static::lazy_static;

use std::collections::HashSet;
use std::sync::Mutex;

lazy_static! {
    static ref HM_LABELS: Mutex<HashSet<u128>> = Mutex::new(HashSet::new());
}

/// Labels are a unique identifier attributed to a block. It represents this
/// block and is unique.
pub struct Label {
    name: String,
}

impl Label {
    /// Return a new, unique label according to the given prefix
    ///
    /// # Example
    ///
    /// ```
    /// use stir::label::Label;
    ///
    /// let l = Label::new("bool");
    ///
    /// assert!(l.name().contains("__bool_"));
    /// ```
    pub fn new(prefix: &str) -> Label {
        Label {
            name: Label::unique_identifier(prefix),
        }
    }

    /// Create a new unique identifier from a given prefix
    fn unique_identifier(prefix: &str) -> String {
        return prefix.to_string();
    }

    /// Return the label's actual name
    pub fn name(&self) -> &String {
        &self.name
    }
}
