use calyx_rs::generation::Grammar;
use std::collections::HashMap;

fn main() {
    let mut grammar = Grammar::new();
    grammar
        .start_single(String::from("{greeting} world."))
        .expect("Error defining start rule");

    grammar
        .weighted_rule(
            String::from("greeting"),
            &HashMap::from([
                (String::from("Hello"), 5.0),
                (String::from("Hi"), 2.0),
                (String::from("Hey"), 2.0),
                (String::from("Yo"), 1.0),
            ]),
        )
        .expect("Error defining greeting rule");

    let text = grammar
        .generate()
        .expect("Error during generation")
        .flatten();

    println!("{}", text);
}
