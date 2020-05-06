use stir::blocks::{Boolean, IfElse, BasicBlock};

fn main() {
    let c = Boolean::new(true);
    let t = Boolean::new(true);
    let f = Boolean::new(false);
    let ie = IfElse::new(
        &c,
        &t,
        Some(&f),
        );

    dbg!(&ie);

    println!("\n{}", ie.output());
}
