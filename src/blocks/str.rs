//! Str are a block wrapper around rust's Strings.

use super::{BasicBlock, Primitive};

use crate::label::Label;

#[derive(Debug)]
pub struct Str {
    label: Label,
    value: String,
}

impl Str {
    pub fn new(value: String) -> Str {
        Str {
            label: Label::new("str"),
            value,
        }
    }
}

impl Primitive for Str {
    type ValueType = String;

    fn set(&mut self, value: Self::ValueType) {
        self.value = value;
    }

    fn get(&self) -> Self::ValueType {
        self.value
    }
}

impl BasicBlock for Str {
    fn label(&self) -> &String {
        self.label.name()
    }

    fn output(&self) -> String {
        self.value.clone()
    }

    fn interpret(&self) -> bool {
        self.value.len() > 0
    }
}
