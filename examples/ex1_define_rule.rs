use calyx_rs::generation::Grammar;

fn main() {
    let mut grammar: Grammar = Grammar::new();

    // This returns a Result to allow you to check for errors during construction, don't just
    // call `expect` or `unwrap` unless you want it to panic!
    grammar
        .start_single(String::from("Hello world."))
        .expect("Error defining start rule");
}
