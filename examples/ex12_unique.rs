use calyx_rs::generation::Grammar;

fn main() {
    let mut grammar = Grammar::new();

    grammar
        .start_single(String::from("{$medal} {$medal} {$medal}"))
        .expect("Error defining start rule");

    grammar
        .uniform_rule(
            String::from("medal"),
            &vec![
                String::from("Gold"),
                String::from("Silver"),
                String::from("Bronze"),
            ],
        )
        .expect("Error defining $medal rule");

    let text = grammar
        .generate()
        .expect("Error during generation")
        .flatten();

    println!("{}", text);
    // > Silver Gold Bronze
}
