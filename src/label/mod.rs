/// Last ID set by the Label framework
static mut LAST_ID: u128 = 0;

/// Labels are a unique identifier attributed to a block. It represents this
/// block and is unique.
#[derive(Debug)]
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
        let mut unique = String::from("__");
        unique.push_str(prefix);
        unique.push_str("_");

        // Get the last ID given and increment it. Then, append it to the
        // unique identifier
        unsafe {
            LAST_ID += 1;
            unique.push_str(&LAST_ID.to_string());
        }

        unique
    }

    /// Return the label's actual name
    pub fn name(&self) -> &String {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_layout() {
        let l = Label::new("a");

        assert!(l.name().contains("__a_"));
    }

    #[test]
    fn id_layout_complex_prefix() {
        let l = Label::new("this_is_complex");

        assert!(l.name().contains("__this_is_complex_"));
    }

    #[test]
    fn last_id_increases() {
        let l0 = Label::new("a");
        let l1 = Label::new("a");

        assert_ne!(l0.name(), l1.name());
    }
}
