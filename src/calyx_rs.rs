mod production;
mod registry;

pub struct Options<R: rand::Rng> {
    strict: bool,
    random_source: R,
}

pub struct Grammar {
    registry: registry::Registry,
}

pub enum Err {
    UndefinedRule { rule_name: String },
    UndefinedFilter { filter_name: String },
}

impl Grammar {
    pub fn start(&self, start_name: &str) -> Result<(), Err> {
        Ok(())
    }
}

impl<R: rand::Rng> Options<R> {
    fn new(strict: bool, random_source: R) -> Options<R> {
        Options {
            strict,
            random_source,
        }
    }

    fn new_lenient(random_source: R) -> Options<R> {
        Options {
            strict: false,
            random_source,
        }
    }

    fn strict(&self) -> bool {
        self.strict
    }
}
