mod production;
mod evaluation;
mod expansion_tree;

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
    ExpandedEmptyBranch,
}

impl Grammar {
    pub fn start(&self, start_name: &str) -> Result<(), CalyxError> {
        Ok(())
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
