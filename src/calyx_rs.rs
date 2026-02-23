use crate::calyx_rs::evaluation::{EvaluationContext, Registry};
use crate::calyx_rs::expansion_tree::{ExpansionTree, ExpansionType};
use rand::SeedableRng;
use rand::prelude::ThreadRng;
use rand::rngs::StdRng;

mod evaluation;
pub mod expansion_tree;
pub mod filter;
mod production;

/// Contains options for grammar generation.
pub struct Options {
    strict: bool,
    random_source: Box<dyn rand::RngCore>,
}

/// Core struct for Calyx grammars
pub struct Grammar {
    registry: Registry,
    options: Options,
}

/// Defines the possible set of errors that may occur during grammar generation.
#[derive(Debug)]
pub enum CalyxError {
    UndefinedRule { rule_name: String },
    UndefinedFilter { filter_name: String },
    DuplicateRule { rule_name: String },
    ExpandedEmptyBranch,
    InvalidExpression { expression: String },
}

impl Grammar {
    /// Creates a new lenient grammar with a local [`ThreadRng`].
    pub fn new() -> Grammar {
        Grammar {
            registry: Registry::new(),
            options: Options::new(false, rand::rng()),
        }
    }

    /// Creates a new lenient grammar with a specified random source.
    pub fn from_rng<R: rand::RngCore + 'static>(random_source: R) -> Grammar {
        Grammar {
            registry: Registry::new(),
            options: Options::new(false, random_source),
        }
    }

    /// Creates a new grammar with the given options.
    pub fn with_options(options: Options) -> Grammar {
        Grammar {
            registry: Registry::new(),
            options,
        }
    }

    pub fn start_single(&mut self, production: String) -> Result<(), CalyxError> {
        self.single_rule("start", production)
    }

    pub fn start_uniform(&mut self, production: Vec<String>) -> Result<(), CalyxError> {
        self.uniform_rule("start", production)
    }

    pub fn single_rule(&mut self, term: &str, production: String) -> Result<(), CalyxError> {
        let branch = vec![production];
        self.registry.define_rule(term, &branch)
    }

    pub fn uniform_rule(&mut self, term: &str, production: Vec<String>) -> Result<(), CalyxError> {
        self.registry.define_rule(term, &production)
    }

    pub fn generate(&mut self) -> Result<ExpansionTree, CalyxError> {
        self.generate_from("start")
    }

    pub fn generate_from(&mut self, start_symbol: &str) -> Result<ExpansionTree, CalyxError> {
        let mut eval_context = EvaluationContext::new(self);
        let start_symbol = start_symbol.to_string();
        let tree = eval_context.expand_and_evaluate(&start_symbol)?;

        Ok(ExpansionTree::chain(ExpansionType::Result, tree))
    }
}

impl Options {
    pub fn new<R: rand::RngCore + 'static>(strict: bool, random_source: R) -> Options {
        Options {
            strict,
            random_source: Box::new(random_source),
        }
    }

    pub fn new_lenient<R: rand::RngCore + 'static>(random_source: R) -> Options {
        Options {
            strict: false,
            random_source: Box::new(random_source),
        }
    }

    pub fn strict(&self) -> bool {
        self.strict
    }

    pub fn lenient(&self) -> bool {
        !self.strict()
    }
}

#[cfg(test)]
mod grammar_tests {
    use crate::calyx_rs::Grammar;

    #[test]
    fn it_works() {
        let mut grammar = Grammar::new();

        assert!(
            grammar
                .start_single("{num} {num} {num}".to_string())
                .is_ok()
        );

        let prod: Vec<String> = vec!["one".to_string(), "two".to_string(), "three".to_string()];
        assert!(grammar.uniform_rule("num", prod).is_ok());

        let result = grammar.generate().unwrap().flatten();
        println!("{result}")
    }

    #[test]
    fn can_filter_memoized_rules() {
        let mut grammar = Grammar::new();

        assert!(grammar.start_single("{@name.lowercase}".to_string()).is_ok());
        assert!(grammar.uniform_rule("name", vec!["Jewels".to_string()]).is_ok());

        let result = grammar.generate();

        let text = result.expect("Error during grammar generation").flatten();

        assert_eq!("jewels", text);
    }
}
