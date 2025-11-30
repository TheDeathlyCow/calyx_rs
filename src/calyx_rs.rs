mod evaluation;
mod expansion_tree;
mod production;

pub struct Options {
    strict: bool,
    random_source: Box<dyn rand::RngCore>,
}

pub struct Grammar {
    registry: evaluation::Registry,
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

    pub fn uniform_rule(
        &mut self,
        term: &str,
        production: Vec<String>,
    ) -> Result<(), CalyxError> {
        self.registry.define_rule(term, &production)
    }
}

impl Options {
    fn new<R: rand::RngCore + 'static>(strict: bool, random_source: R) -> Options {
        Options {
            strict,
            random_source: Box::new(random_source),
        }
    }

    fn new_lenient<R: rand::RngCore + 'static>(random_source: R) -> Options {
        Options {
            strict: false,
            random_source: Box::new(random_source),
        }
    }

    fn strict(&self) -> bool {
        self.strict
    }
}
