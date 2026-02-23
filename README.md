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

All the examples shown here can be found in the [examples](./examples) directory, to allow you to play around with them
as you please.

## Generation Basics

Add the library to your `Cargo.toml` and construct a `Grammar` to define rules and generate text.

```rust
use calyx_rs::generation::Grammar;

fn main() {
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

fn main() {
    let mut grammar: Grammar = Grammar::new();
    grammar
        .start_single(String::from("Hello world."))
        .expect("Error defining start rule");

    let expansion: Result<ExpansionTree, CalyxError> = grammar.generate();
    let text: String = expansion.expect("Error during generation").flatten();

    println!("{}", text);
    // > Hello world.
}
```

Obviously, this hardcoded sentence isn’t very interesting by itself. Possible variations can be added to the text by
adding additional rules which provide a named set of text strings. The rule delimiter syntax (`{}`) can be used to
substitute the generated content of other rules.

```rust
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
```

Each time `generate()` runs, it evaluates the tree and randomly selects variations of rules to construct a resulting
string.

<details>
<summary>Show Example</summary>

```rust
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
    println!("{}", text);
    // > Hey world.
    println!("{}", text);
    // > Hi world.
    println!("{}", text);
    // > Yo world.
}
```

</details>

In the previous example, the different greetings were picked randomly on each generation with a uniform distribution.
However, we can also supply a custom weighted distribution for the different greetings:

<details>
<summary>Show Example</summary>

```rust
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
```

</details>

In this case, the grammar will pick "Hello" 50% of the time, "Hi" and "Hey" 20% of the time, and "Yo" 10% of the time
when greeting is expanded.

By convention, the `start` rule specifies the default starting point for generating the final text. You can start from
any other named rule by passing it explicitly to the `generate_from()` method.

<details>
<summary>Show Example</summary>

```rust
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
```

</details>

## Template Expressions

Basic rule substitution uses single curly brackets as delimiters for template expressions:

<details>
<summary>Show Example</summary>

```rust
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
```

</details>

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

Dot-notation is supported in template expressions, allowing you to call a variety of different processing functions on
the string returned from a rule.

<details>
<summary>Show Example</summary>

```rust
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
```

</details>

Multiple filters can also be chained onto the same rule, and are evaluated left to right:

<details>
<summary>Show Example</summary>

```rust
use calyx_rs::generation::Grammar;

fn main() {
    let mut grammar = Grammar::new();

    grammar
        .start_single(String::from("{greeting.uppercase.lowercase} there"))
        .expect("Error defining start rule");

    grammar
        .single_rule(String::from("greeting"), String::from("hello"))
        .expect("Error defining greeting rule");

    let text = grammar
        .generate()
        .expect("Error during generation")
        .flatten();

    println!("{}", text);
    // > hello there
}
```

</details>

The full set of builtin filter functions is defined in [`filter.rs`](./src/generation/filter.rs).

## Memoized Rules

Rule expansions can be 'memoized' so that multiple references to the same rule return the same value. This is useful for
picking a noun from a list and reusing it in multiple places within a text.

The `@` sigil is used to mark memoized rules. This evaluates the rule and stores it in memory the first time it's
referenced. All subsequent references to the memoized rule use the same stored value.

[See Example Here](./examples/ex11_memo.rs)

## Unique Rules

Rule expansions can be marked as 'unique', meaning that multiple references to the same rule always return a different
value. This is useful for situations where the same result appearing twice would appear awkward and messy.

Unique rules are marked by the `$` sigil.

[See Example Here](./examples/ex12_unique.rs)

## Dynamically Constructing Rules

Calyx Rust does not currently support this feature.

## External File Formats

Calyx Rust does not currently support this feature.