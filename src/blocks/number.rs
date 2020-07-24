//! This block represents a number. Number are represented using a double

use super::{BasicBlock, Primitive};

use crate::label::Label;

#[derive(Debug)]
pub struct Number {
    label: Label,
    value: f64,
}

impl Number {
    pub fn new(value: f64) -> Number {
        Number {
            label: Label::new("number"),
            value,
        }
    }
}

impl Primitive for Number {
    type ValueType = f64;

    fn get(&self) -> Self::ValueType {
        self.value
    }

    fn set(&mut self, value: Self::ValueType) {
        self.value = value;
    }
}

impl BasicBlock for Number {
    fn label(&self) -> &String {
        self.label.name()
    }

    fn output(&self) -> String {
        self.value.to_string()
    }

    fn interpret(&self) -> bool {
        !self.value.is_nan()
    }
}
