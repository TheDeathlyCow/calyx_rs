use calyx_rs::generation::Grammar;

fn main() {
    println!("==Without memoization==");
    let mut grammar = Grammar::new();

    grammar
        .start_single(String::from("{name} <{name.lowercase}>"))
        .expect("Error defining start rule");

    grammar
        .uniform_rule(
            String::from("name"),
            &vec![
                String::from("Daenerys"),
                String::from("Tyrion"),
                String::from("Jon"),
            ],
        )
        .expect("Error defining name rule");

    for _ in 0..3 {
        let text = grammar
            .generate()
            .expect("Error during generation")
            .flatten();
        println!("{}", text);
    }
    // > Tyrion <jon>
    // > Daenerys <tyrion>
    // > Jon <daenerys>


    println!();
    println!("==With memoization==");

    let mut grammar = Grammar::new();

    grammar
        .start_single(String::from("{@name} <{@name.lowercase}>"))
        .expect("Error defining start rule");

    grammar
        .uniform_rule(
            String::from("name"),
            &vec![
                String::from("Daenerys"),
                String::from("Tyrion"),
                String::from("Jon"),
            ],
        )
        .expect("Error defining name rule");

    for _ in 0..3 {
        let text = grammar
            .generate()
            .expect("Error during generation")
            .flatten();
        println!("{}", text);
    }
    // > Tyrion <tyrion>
    // > Daenerys <daenerys>
    // > Jon <jon>
}
