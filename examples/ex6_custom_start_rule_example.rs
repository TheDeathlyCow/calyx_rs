use calyx_rs::generation::Grammar;

fn main() {
    let mut grammar = Grammar::new();

    grammar
        .single_rule(String::from("hello"), String::from("Hello world."))
        .expect("Error defining greeting rule");

    let text = grammar
        .generate_from(&String::from("hello"))
        .expect("Error during generation")
        .flatten();

    println!("{}", text);
    // > Hello world.
}
