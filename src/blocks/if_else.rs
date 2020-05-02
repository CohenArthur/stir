use super::BasicBlock;

use crate::label::Label;

/// An IfElse block allows you to execute another block based on a given
/// condition
pub struct IfElse<'block> {
    /// Label of the IfElse block
    label: Label,

    /// Block of the condition. If the Block evaluates to `true`, execute the
    /// `t_block`. Else, execute the `f_block`
    cond_block: &'block dyn BasicBlock,

    /// True block. Executed if `cond_block` evaluates to `true`
    t_block: &'block dyn BasicBlock,

    /// False block. Executed if `cond_block` evaluates to `false`
    f_block: Option<&'block dyn BasicBlock>,
}

impl<'block> IfElse<'block> {
    /// Create a new IfElse block. If there is no body to the `else`,
    /// pass `None` as argument
    pub fn new(
        cond_block: &'block dyn BasicBlock,
        t_block: &'block dyn BasicBlock,
        f_block: Option<&'block dyn BasicBlock>,
    ) -> IfElse<'block> {
        IfElse {
            label: Label::new("if_else"),
            cond_block,
            t_block,
            f_block,
        }
    }
}

impl BasicBlock for IfElse<'_> {
    fn label(&self) -> &String {
        return self.label.name();
    }

    fn debug(&self) {
        dbg!(self);
    }

    fn output(&self) -> String {
        String::from("if .. else ..") // FIXME: Logic: Add actual printing
    }

    fn interpret(&self) -> bool {
        if self.cond_block.interpret() {
            self.t_block.interpret()
        } else {
            match self.f_block {
                Some(f_b) => f_b.interpret(),
                None => false,
            }
        }
    }
}

impl std::fmt::Debug for IfElse<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "// -- begin IfElse").unwrap();

        dbg!(&self.label);

        write!(f, "// -- end IfElse")
    }
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn cond_true() {
        let ie = IfElse::new(
            Boolean::new(true),
            Boolean::new(true),
            Some(Boolean::new(false)),
        );

        assert!(ie.interpret());
    }

    #[test]
    fn cond_false() {
        let ie = IfElse::new(
            Boolean::new(false),
            Boolean::new(false),
            Some(Boolean::new(true)),
        );

        assert!(ie.interpret());
    }

    #[test]
    fn no_else() {
        let ie = IfElse::new(Boolean::new(false), Boolean::new(false), None);

        assert!(!ie.interpret());
    }
}
