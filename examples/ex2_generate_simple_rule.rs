use calyx_rs::generation::expansion_tree::ExpansionTree;
use calyx_rs::generation::{CalyxError, Grammar};

fn main() {
    let mut grammar: Grammar = Grammar::new();
    grammar
        .start_single(String::from("Hello world."))
        .expect("Error defining start rule");

    let expansion: Result<ExpansionTree, CalyxError> = grammar.generate();
    let text: String = expansion.expect("Error during generation").flatten();

    println!("{}", text);
    // > Hello world.
}
