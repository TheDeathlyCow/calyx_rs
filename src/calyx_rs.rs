use crate::calyx_rs::evaluation::{EvaluationContext, Registry};
use crate::calyx_rs::expansion_tree::{ExpansionTree, ExpansionType};
use std::collections::HashMap;

mod evaluation;
pub mod expansion_tree;
pub mod filter;
mod production;

/// Contains options for grammar generation.
pub struct Options {
    strict: bool,
    random_source: Box<dyn rand::RngCore>,
}

/// Core struct for Calyx grammars. See the README for more guidance on the format of productions.
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
    InvalidWeight { weight: f64 },
}

impl Grammar {
    /// Creates a new lenient grammar with a local [`ThreadRng`].
    pub fn new() -> Grammar {
        Grammar {
            registry: Registry::new(),
            options: Options::new(false, rand::rng()),
        }
    }

    /// Creates a new strict grammar with a local [`ThreadRng`].
    pub fn new_strict() -> Grammar {
        Grammar {
            registry: Registry::new(),
            options: Options::new(true, rand::rng()),
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
    pub fn from_options(options: Options) -> Grammar {
        Grammar {
            registry: Registry::new(),
            options,
        }
    }

    /// Defines a new single expansion of the `start` rule.
    ///
    /// # Errors
    /// - [CalyxError::DuplicateRule] if the start rule is already defined.
    /// - [CalyxError::InvalidExpression] if the production could not be parsed.
    ///
    pub fn start_single(&mut self, production: String) -> Result<(), CalyxError> {
        self.single_rule(String::from("start"), production)
    }

    /// Defines a new uniform expansion of the `start` rule.
    ///
    /// # Errors
    /// - [CalyxError::DuplicateRule] if the start rule is already defined.
    /// - [CalyxError::InvalidExpression] if the production could not be parsed.
    ///
    pub fn start_uniform(&mut self, production: &Vec<String>) -> Result<(), CalyxError> {
        self.uniform_rule(String::from("start"), production)
    }

    /// Defines a new uniform expansion of the `start` rule.
    ///
    /// # Errors
    /// - [CalyxError::DuplicateRule] if the start rule is already defined.
    /// - [CalyxError::InvalidExpression] if the production could not be parsed.
    ///
    pub fn start_weighted(&mut self, production: &HashMap<String, f64>) -> Result<(), CalyxError> {
        self.weighted_rule(String::from("start"), production)
    }

    /// Defines a new single expansion of the given term.
    ///
    /// # Errors
    /// - [CalyxError::DuplicateRule] if the term is already defined.
    /// - [CalyxError::InvalidExpression] if the production could not be parsed.
    /// - [CalyxError::InvalidWeight] if any weight in the production is not [finite](f64::is_finite).
    ///
    pub fn single_rule(&mut self, term: String, production: String) -> Result<(), CalyxError> {
        let branch = vec![production];
        self.registry.define_rule(term, &branch)
    }

    /// Defines a new uniform expansion of the given term.
    ///
    /// # Errors
    /// - [CalyxError::DuplicateRule] if the term is already defined.
    /// - [CalyxError::InvalidExpression] if the production could not be parsed.
    ///
    pub fn uniform_rule(
        &mut self,
        term: String,
        production: &Vec<String>,
    ) -> Result<(), CalyxError> {
        self.registry.define_rule(term, production)
    }

    /// Defines a new weighted expansion of the given term.
    ///
    /// # Errors
    /// - [CalyxError::DuplicateRule] if the term is already defined.
    /// - [CalyxError::InvalidExpression] if the production could not be parsed.
    /// - [CalyxError::InvalidWeight] if any weight in the production is not [finite](f64::is_finite).
    ///
    pub fn weighted_rule(
        &mut self,
        term: String,
        production: &HashMap<String, f64>,
    ) -> Result<(), CalyxError> {
        self.registry.define_weighted_rule(term, production)
    }

    /// Generate an expansion of this grammar, starting from the rule named `start`.
    ///
    /// # Errors
    ///
    /// - [CalyxError::UndefinedRule] if attempting to expand a rule that is not defined, and the
    /// grammar options are [Options::strict].
    /// - [CalyxError::UndefinedFilter] if attempting to apply a filter to an expansion that does
    /// not exist.
    /// - [CalyxError::ExpandedEmptyBranch] if attempting to expand a branch production and that
    /// production has no children.
    ///
    /// # Examples
    ///
    /// ```
    /// use calyx_rs::calyx_rs::expansion_tree::ExpansionTree;
    /// use calyx_rs::calyx_rs::{CalyxError, Grammar};
    ///
    /// let mut grammar: Grammar = Grammar::new();
    /// assert!(grammar.start_single(String::from("{odd_number} {even_number}")).is_ok());
    /// assert!(grammar.single_rule(String::from("odd_number"), String::from("1")).is_ok());
    /// assert!(grammar.single_rule(String::from("even_number"), String::from("2")).is_ok());
    ///
    /// let result: Result<ExpansionTree, CalyxError> = grammar.generate();
    ///
    /// let expansion: ExpansionTree = result.expect("Error during generation");
    /// let text: String = expansion.flatten();
    /// assert_eq!(text, "1 2");
    /// ```
    ///
    pub fn generate(&mut self) -> Result<ExpansionTree, CalyxError> {
        self.generate_from(&String::from("start"))
    }

    /// Generate an expansion of this grammar, starting from a given start symbol.
    ///
    /// # Errors
    ///
    /// - [CalyxError::UndefinedRule] if attempting to expand a rule that is not defined, and the
    /// grammar options are [Options::strict].
    /// - [CalyxError::UndefinedFilter] if attempting to apply a filter to an expansion that does
    /// not exist.
    /// - [CalyxError::ExpandedEmptyBranch] if attempting to expand a branch production and that
    /// production has no children.
    ///
    /// # Examples
    ///
    /// ```
    /// use calyx_rs::calyx_rs::expansion_tree::ExpansionTree;
    /// use calyx_rs::calyx_rs::{CalyxError, Grammar};
    ///
    /// let mut grammar: Grammar = Grammar::new();
    /// assert!(grammar.start_single(String::from("{odd_number} {even_number}")).is_ok());
    /// assert!(grammar.single_rule(String::from("odd_number"), String::from("1")).is_ok());
    /// assert!(grammar.single_rule(String::from("even_number"), String::from("2")).is_ok());
    ///
    /// let result: Result<ExpansionTree, CalyxError> = grammar.generate_from(&String::from("odd_number"));
    ///
    /// let expansion: ExpansionTree = result.expect("Error during generation");
    /// let text: String = expansion.flatten();
    /// assert_eq!(text, "1");
    /// ```
    ///
    pub fn generate_from(&mut self, start_symbol: &String) -> Result<ExpansionTree, CalyxError> {
        let mut eval_context = EvaluationContext::new(self);
        let tree = eval_context.expand_and_evaluate(start_symbol)?;

        Ok(ExpansionTree::chain(ExpansionType::Result, tree))
    }
}

impl Options {
    /// Creates a new options struct with a defined [Self::strict] mode and random source.
    pub fn new<R: rand::RngCore + 'static>(strict: bool, random_source: R) -> Options {
        Options {
            strict,
            random_source: Box::new(random_source),
        }
    }

    /// Creates a new [Self::lenient] options struct,
    pub fn new_lenient<R: rand::RngCore + 'static>(random_source: R) -> Options {
        Options {
            strict: false,
            random_source: Box::new(random_source),
        }
    }

    /// Strict mode requires all rules to be defined in a grammar to successfully evaluate it.
    ///
    /// Strict is the opposite of [Self::lenient].
    pub fn strict(&self) -> bool {
        self.strict
    }

    /// In lenient mode, undefined rules in a grammar will evaluate to an empty string.
    ///
    /// Lenient is the opposite of [Self::strict]
    pub fn lenient(&self) -> bool {
        !self.strict()
    }
}

#[cfg(test)]
mod grammar_tests {
    use crate::calyx_rs::expansion_tree::ExpansionType;
    use crate::calyx_rs::{CalyxError, Grammar};
    use rand::SeedableRng;
    use rand::rngs::StdRng;
    use std::collections::HashMap;

    #[test]
    fn evaluate_start_rule() {
        let mut grammar = Grammar::new();

        assert!(
            grammar
                .uniform_rule(String::from("start"), &vec![String::from("atom")])
                .is_ok()
        );

        let expansion = grammar.generate().expect("Error during grammar generation");
        assert!(matches!(expansion.symbol(), ExpansionType::Result));
        assert_eq!(expansion.flatten(), "atom");
    }

    #[test]
    fn expand_empty_uniform_branch_fails() {
        let mut grammar = Grammar::new();

        assert!(grammar.start_uniform(&vec![]).is_ok());

        let result = grammar.generate();
        assert!(matches!(result, Err(CalyxError::ExpandedEmptyBranch)));
    }

    #[test]
    fn define_empty_weighted_branch_fails() {
        let mut grammar = Grammar::new();

        let result = grammar.start_weighted(&HashMap::new());
        assert!(matches!(result, Err(CalyxError::InvalidWeight { weight }) if weight == 0.0))
    }

    #[test]
    fn evaluate_recursive_rule() {
        let rng = StdRng::seed_from_u64(12345);
        let mut grammar = Grammar::from_rng(rng);

        assert!(
            grammar
                .start_single(String::from("{num} {num} {num}"))
                .is_ok()
        );

        assert!(
            grammar
                .uniform_rule(
                    String::from("num"),
                    &vec![
                        String::from("one"),
                        String::from("two"),
                        String::from("three")
                    ]
                )
                .is_ok()
        );

        let text = grammar
            .generate()
            .expect("Error during grammar generation")
            .flatten();
        assert_eq!(text, "one three three");
    }

    #[test]
    fn evaluate_weighted_rule() {
        let rng = StdRng::seed_from_u64(12345);
        let mut grammar = Grammar::from_rng(rng);

        assert!(
            grammar
                .start_single(String::from("{num} {num} {num} {num} {num} {num}"))
                .is_ok()
        );

        assert!(
            grammar
                .weighted_rule(
                    String::from("num"),
                    &HashMap::from([
                        (String::from("1"), 1.0),
                        (String::from("2"), 2.0),
                        (String::from("3"), 3.0),
                    ])
                )
                .is_ok()
        );

        let text = grammar
            .generate()
            .expect("Error during grammar generation")
            .flatten();
        assert_eq!(text, "3 3 3 3 3 1");
    }

    #[test]
    fn parse_negative_weight_fails() {
        let rng = StdRng::seed_from_u64(12345);
        let mut grammar = Grammar::from_rng(rng);

        assert!(
            grammar
                .start_single(String::from("{num} {num} {num} {num} {num} {num}"))
                .is_ok()
        );

        assert!(
            grammar
                .weighted_rule(
                    String::from("num"),
                    &HashMap::from([
                        (String::from("1"), -1.0),
                        (String::from("2"), 2.0),
                        (String::from("3"), 3.0),
                    ])
                )
                .is_err()
        );
    }

    #[test]
    fn parse_zero_weight_fails() {
        let rng = StdRng::seed_from_u64(12345);
        let mut grammar = Grammar::from_rng(rng);

        assert!(
            grammar
                .start_single(String::from("{num} {num} {num} {num} {num} {num}"))
                .is_ok()
        );

        assert!(
            grammar
                .weighted_rule(
                    String::from("num"),
                    &HashMap::from([
                        (String::from("1"), 0.0),
                        (String::from("2"), 2.0),
                        (String::from("3"), 3.0),
                    ])
                )
                .is_err()
        );
    }

    #[test]
    fn strict_options_return_unknown_rule_error() {
        let mut grammar = Grammar::new_strict();

        assert!(grammar.start_single(String::from("{name}")).is_ok());

        let result = grammar.generate();

        assert!(
            matches!(result, Err(CalyxError::UndefinedRule {ref rule_name}) if rule_name == "name"),
            "Expected UndefinedRule('name'), but got {:?}",
            result
        )
    }

    #[test]
    fn memoized_rules_return_identical_expression() {
        let rng = StdRng::seed_from_u64(12345);
        let mut grammar = Grammar::from_rng(rng);

        assert!(
            grammar
                .start_single(String::from("{@name} {@name} {@name} {@name} {@name}"))
                .is_ok()
        );
        assert!(
            grammar
                .uniform_rule(
                    String::from("name"),
                    &vec![String::from("Jewels"), String::from("Joey")]
                )
                .is_ok()
        );

        let result = grammar.generate();
        let text = result.expect("Error during grammar generation").flatten();

        assert_eq!("Jewels Jewels Jewels Jewels Jewels", text);
    }

    #[test]
    fn can_filter_memoized_rules() {
        let mut grammar = Grammar::new();

        assert!(
            grammar
                .start_single(String::from("{@name.lowercase}"))
                .is_ok()
        );
        assert!(
            grammar
                .uniform_rule(String::from("name"), &vec![String::from("Jewels")])
                .is_ok()
        );

        let result = grammar.generate();
        let text = result.expect("Error during grammar generation").flatten();

        assert_eq!("jewels", text);
    }
}
