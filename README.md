# Calyx: Rust

A Rust port of the [Calyx](https://github.com/maetl/calyx) generative grammar engine.

# Development

Calyx Rust is built with Cargo, which comes with the standard distribution of [Rust](https://rust-lang.org/).

## Build

```
cargo build

# build with optimizations
cargo build --release
```

## Run Tests

```
cargo test
```

# License

A Rust port of the Calyx generative grammar library
Copyright (C) TheDeathlyCow

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.

# Usage and Examples

## Generation Basics

Add the library to your `Cargo.toml` and construct a `Grammar` to define rules and generate text.

```rust
use calyx_rs::generation::Grammar;

#[test]
fn define_rule() {
    let mut grammar: Grammar = Grammar::new();

    // This returns a Result to allow you to check for errors during construction, don't just
    // call `expect` or `unwrap` unless you want it to panic!
    grammar
        .start_single(String::from("Hello world."))
        .expect("Error defining start rule");
}
```

Once the grammar is made, call the `generate()` method to retrieve a randomly generated result. This result contains a
tree representation of the generation output, but it can be converted to text with `flatten()`:

```rust 
use calyx_rs::generation::expansion_tree::ExpansionTree;
use calyx_rs::generation::{CalyxError, Grammar};

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
```

Obviously, this hardcoded sentence isn’t very interesting by itself. Possible variations can be added to the text by
adding additional rules which provide a named set of text strings. The rule delimiter syntax (`{}`) can be used to
substitute the generated content of other rules.

```rust
use calyx_rs::generation::expansion_tree::ExpansionTree;
use calyx_rs::generation::{CalyxError, Grammar};

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
```

Each time `generate()` runs, it evaluates the tree and randomly selects variations of rules to construct a resulting
string.

```rust
use calyx_rs::generation::expansion_tree::ExpansionTree;
use calyx_rs::generation::{CalyxError, Grammar};

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
```

In the previous example, the different greetings were picked randomly on each generation with a uniform distribution.
However, we can also supply a custom weighted distribution for the different greetings:

```rust
// ...
use std::collections::HashMap;

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
```

In this case, the grammar will pick "Hello" 50% of the time, "Hi" and "Hey" 20% of the time, and "Yo" 10% of the time
when greeting is expanded.

By convention, the `start` rule specifies the default starting point for generating the final text. You can start from
any other named rule by passing it explicitly to the `generate_from()` method.

```rust
// ...

#[test]
fn custom_start_rule_example() {
    let mut grammar = Grammar::new();

    grammar
        .single_rule(
            String::from("hello"),
            String::from("Hello world.")
        )
        .expect("Error defining greeting rule");

    let text = grammar
        .generate_from(&String::from("hello"))
        .expect("Error during generation")
        .flatten();

    assert_eq!(text, "Hello world.");
}
```

## Template Expressions

Basic rule substitution uses single curly brackets as delimiters for template expressions:

```rust
// ...

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
```

## Random Sampling

Calyx allows for you to use any type that implements the `rand::RngCore` trait from
the [rand](https://docs.rs/rand/latest/rand/) library. A seeded grammar can be constructed using a custom set of
`Options`.

```rust
// ...
use rand::prelude::StdRng;
use rand::SeedableRng;

#[test]
fn rng_example() {
    let rng = StdRng::seed_from_u64(12345);
    let grammar = Grammar::from_rng(rng);
}
```

The default generator used will be a handle to the local `ThreadRng`.

## Filters

TODO

## Memoized Rules

## Unique Rules
