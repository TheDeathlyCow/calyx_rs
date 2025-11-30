use crate::calyx_rs::evaluation::{EvaluationContext, Registry};
use crate::calyx_rs::expansion_tree::{ExpansionTree, ExpansionType};

mod evaluation;
mod expansion_tree;
mod production;

pub struct Options {
    strict: bool,
    random_source: Box<dyn rand::RngCore>,
}

pub struct Grammar {
    registry: Registry,
    options: Options,
}

#[derive(Debug)]
pub enum CalyxError {
    UndefinedRule { rule_name: String },
    UndefinedFilter { filter_name: String },
    DuplicateRule { rule_name: String },
    ExpandedEmptyBranch,
    InvalidExpression { expression: String },
}

impl Grammar {
    pub fn new(options: Options) -> Grammar {
        let registry = Registry::new();
        Grammar { registry, options }
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
}
