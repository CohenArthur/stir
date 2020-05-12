//! A Call block is a block whose aim is to call a function. It has arguments,
//! as well as a return value. It takes a vector of arguments and can only
//! return one value at a time.

use super::BasicBlock;

use crate::label::Label;

#[derive(Debug)]
pub struct Call<'block> {
    label: Label,
    function_label: &'block Label,
    args: &'block Vec<&'block Label>, // FIXME: Use better representation than Labels
    return_value: &'block Label,
}

impl<'block> Call<'block> {
}

impl BasicBlock for Call<'_> {
    fn label(&self) -> &String {
        self.label.name()
    }

    fn interpret(&self) -> bool {
        true // FIXME: Add logic
    }

    fn output(&self) -> String {
        "Call".to_string() // FIXME: Add logic
    }
}
