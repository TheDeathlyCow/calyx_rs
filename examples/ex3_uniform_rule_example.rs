use calyx_rs::generation::Grammar;

fn main() {
    let mut grammar = Grammar::new();
    grammar
        .start_single(String::from("{greeting} world."))
        .expect("Error defining start rule");

    grammar
        .uniform_rule(
            String::from("greeting"),
            &vec![
                String::from("Hello"),
                String::from("Hi"),
                String::from("Hey"),
                String::from("Yo"),
            ],
        )
        .expect("Error defining greeting rule");

    let text = grammar
        .generate()
        .expect("Error during generation")
        .flatten();

    println!("{}", text);
    // > Hello world.
}
