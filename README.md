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

[See Example Here](./examples/ex1_define_rule.rs)

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

[See Example Here](./examples/ex2_generate_simple_rule.rs)

Obviously, this hardcoded sentence isn’t very interesting by itself. Possible variations can be added to the text by
adding additional rules which provide a named set of text strings. The rule delimiter syntax (`{}`) can be used to
substitute the generated content of other rules.

[See Example Here](./examples/ex3_uniform_rule_example.rs)

Each time `generate()` runs, it evaluates the tree and randomly selects variations of rules to construct a resulting
string.

[See Example Here](./examples/ex4_uniform_rule_multiple_generation_example.rs)

In the previous example, the different greetings were picked randomly on each generation with a uniform distribution.
However, we can also supply a custom weighted distribution for the different greetings:

[See Example Here](./examples/ex5_weighted_rule_example.rs)

In this case, the grammar will pick "Hello" 50% of the time, "Hi" and "Hey" 20% of the time, and "Yo" 10% of the time
when greeting is expanded.

By convention, the `start` rule specifies the default starting point for generating the final text. You can start from
any other named rule by passing it explicitly to the `generate_from()` method.

[See Example Here](./examples/ex6_custom_start_rule_example.rs)

## Template Expressions

Basic rule substitution uses single curly brackets as delimiters for template expressions:

[See Example Here](./examples/ex7_random_fruit.rs)

## Random Sampling

Calyx allows for you to use any type that implements the `rand::RngCore` trait from
the [rand](https://docs.rs/rand/latest/rand/) library. A seeded grammar can be constructed using a custom set of
`Options`.

```rust
use calyx_rs::generation::Grammar;
use rand::SeedableRng;
use rand::prelude::StdRng;

fn main() {
    let rng = StdRng::seed_from_u64(12345);
    let grammar = Grammar::from_rng(rng);
}
```

[See Example Here](./examples/ex8_seeded_rng.rs)

The default generator used will be a handle to the local `ThreadRng`.

## Filters

Dot-notation is supported in template expressions, allowing you to call a variety of different processing functions on
the string returned from a rule.

[See Example Here](./examples/ex9_filters.rs)

Multiple filters can also be chained onto the same rule, and are evaluated left to right:

[See Example Here](./examples/ex10_multiple_filters.rs)

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