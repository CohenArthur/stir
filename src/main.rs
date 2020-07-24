use stir::blocks::*;
use stir::recipe::Recipe;

fn main() {
    let c = Boolean::new(true);
    let t = Boolean::new(true);
    let f = Boolean::new(false);
    let ie = IfElse::new(&c, &t, Some(&f));

    let num0 = Number::new(12f64);
    let num1 = Number::new(13f64);
    let num2 = Number::new(16f64);

    let an_str = Str::new(String::from("Clackotte best bot"));

    let ie_num = IfElse::new(&num0, &num1, Some(&num2));

    let l = Loop::new(None, None, None);

    let b = Boolean::new(true);

    let mega_l = Loop::new(None, Some(&b), Some(&ie));

    let mut vec: Vec<&dyn BasicBlock> = Vec::new();
    vec.push(&mega_l);
    vec.push(&l);
    vec.push(&b);
    vec.push(&ie);

    let func = Function::new(None, &vec);
    let call = Call::new(&func, None);

    let mut recipe = Recipe::new();

    recipe.add(&func);
    recipe.add(&call);
    recipe.add(&ie_num);
    recipe.add(&an_str);

    println!("{}", recipe.output());
}
