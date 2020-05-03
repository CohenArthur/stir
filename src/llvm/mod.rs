//! The LLVM Module exposes a trait that blocks wanting to translate to LLVM
//! shall implement

pub mod error;

use error::LlvmTranslateError;

pub trait LlvmTranslate {
    fn translate(&self) -> Result<(), LlvmTranslateError>;
}
