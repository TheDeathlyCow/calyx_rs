pub mod generation;

#[cfg(test)]
mod readme_examples {
    use crate::generation::expansion_tree::ExpansionTree;
    use crate::generation::{CalyxError, Grammar};
    use std::collections::HashMap;
    use rand::prelude::StdRng;
    use rand::SeedableRng;

    #[test]
    fn define_rule() {
        let mut grammar: Grammar = Grammar::new();

        // This returns a Result to allow you to check for errors during construction, don't just
        // call `expect` or `unwrap` unless you want it to panic!
        grammar
            .start_single(String::from("Hello world."))
            .expect("Error defining start rule");
    }

    #[test]
    fn generate_simple_rule() {
        let mut grammar: Grammar = Grammar::new();
        grammar
            .start_single(String::from("Hello world."))
            .expect("Error defining start rule");

        let expansion: Result<ExpansionTree, CalyxError> = grammar.generate();
        let text: String = expansion.expect("Error during generation").flatten();

        assert_eq!(text, "Hello world.");
    }

    #[test]
    fn uniform_rule_example() {
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

    #[test]
    fn uniform_rule_multiple_generation_example() {
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
        println!("{}", text);
        // > Hey world.
        println!("{}", text);
        // > Hi world.
        println!("{}", text);
        // > Yo world.
    }

    #[test]
    fn weighted_rule_example() {
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

    #[test]
    fn custom_start_rule_example() {
        let mut grammar = Grammar::new();

        grammar
            .single_rule(String::from("hello"), String::from("Hello world."))
            .expect("Error defining greeting rule");

        let text = grammar
            .generate_from(&String::from("hello"))
            .expect("Error during generation")
            .flatten();

        assert_eq!(text, "Hello world.");
    }

    #[test]
    fn random_fruit() {
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

    #[test]
    fn rng_example() {
        let rng = StdRng::seed_from_u64(12345);
        let grammar = Grammar::from_rng(rng);
    }
}
