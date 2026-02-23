use calyx_rs::generation::Grammar;

fn main() {
    let mut grammar = Grammar::new();

    grammar
        .start_single(String::from("{colour} {fruit}"))
        .expect("Error defining start rule");

    grammar
        .uniform_rule(
            String::from("colour"),
            &vec![
                String::from("red"),
                String::from("green"),
                String::from("yellow"),
            ],
        )
        .expect("Error defining colour rule");

    grammar
        .uniform_rule(
            String::from("fruit"),
            &vec![
                String::from("apple"),
                String::from("pear"),
                String::from("tomato"),
            ],
        )
        .expect("Error defining colour rule");

    for _ in 0..6 {
        let text = grammar
            .generate()
            .expect("Error during generation")
            .flatten();

        print!("{}", text);
    }

    // > "yellow pear"
    // > "red apple"
    // > "green tomato"
    // > "red pear"
    // > "yellow tomato"
    // > "green apple"
}
