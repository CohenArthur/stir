//! Blocks are the building... blocks of a STIR program
//! All types of blocks implement the
//! [`BasicBlock`](blocks/trait.BasicBlock.html) trait and have a ::new()
//! method for easy initialization

mod basic_block;
mod boolean;
mod call;
mod critical;
mod function;
mod if_else;
mod r#loop;

pub use basic_block::BasicBlock;
pub use boolean::Boolean;
pub use call::Call;
pub use critical::Critical;
pub use function::Function;
pub use if_else::IfElse;
pub use r#loop::Loop;
