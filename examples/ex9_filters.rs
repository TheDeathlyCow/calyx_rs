use calyx_rs::generation::Grammar;

fn main() {
    let mut grammar = Grammar::new();

    grammar
        .start_single(String::from("{greeting.uppercase} there"))
        .expect("Error defining start rule");

    grammar
        .single_rule(String::from("greeting"), String::from("hello"))
        .expect("Error defining greeting rule");

    let text = grammar
        .generate()
        .expect("Error during generation")
        .flatten();

    println!("{}", text);
    // > HELLO there
}
