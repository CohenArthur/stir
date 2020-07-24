use stir::blocks::*;

fn main() {
    let c = Boolean::new(true);
    let t = Boolean::new(true);
    let f = Boolean::new(false);
    let ie = IfElse::new(&c, &t, Some(&f));

    let l = Loop::new(None, None, None);

    let b = Boolean::new(true);

    let mega_l = Loop::new(None, Some(&b), Some(&ie));

    dbg!(l);
    dbg!(mega_l);
    dbg!(b);
    dbg!(ie);
}
