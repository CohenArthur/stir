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
        let mut s = String::from("IF ");
        s.push_str(&self.cond_block.output());
        s.push_str(" {\n");
        s.push_str(&self.t_block.output());
        s.push_str("\n}");

        match self.f_block {
            Some(else_block) => {
                s.push_str(" ELSE {\n");
                s.push_str(&else_block.output());
                s.push_str("\n}\n");
                s
            }
            None => {
                s.push('\n');
                s
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::blocks::Boolean;

    #[test]
    fn cond_true() {
        let c = Boolean::new(true);
        let t = Boolean::new(true);
        let f = Boolean::new(false);
        let ie = IfElse::new(
            &c,
            &t,
            Some(&f),
        );

        assert!(ie.interpret());
    }

    #[test]
    fn cond_false() {
        let c = Boolean::new(true);
        let t = Boolean::new(true);
        let f = Boolean::new(false);
        let ie = IfElse::new(
            &c,
            &t,
            Some(&f),
        );

        assert!(ie.interpret());
    }

    #[test]
    fn no_else() {
        let c = Boolean::new(true);
        let t = Boolean::new(false);
        let ie = IfElse::new(
            &c,
            &t,
            None,
        );

        assert!(!ie.interpret());
    }

    #[test]
    fn output_complete() {
        let ie_str = "IF true {\ntrue\n} ELSE {\nfalse\n}\n";

        let c = Boolean::new(true);
        let t = Boolean::new(true);
        let f = Boolean::new(false);
        let ie = IfElse::new(
            &c,
            &t,
            Some(&f),
        );

        assert_eq!(ie_str, ie.output());
    }

    #[test]
    fn output_no_else() {
        let ie_str = "IF true {\nfalse\n}\n";

        let c = Boolean::new(true);
        let t = Boolean::new(false);
        let ie = IfElse::new(
            &c,
            &t,
            None
        );

        assert_eq!(ie_str, ie.output());
    }
}
