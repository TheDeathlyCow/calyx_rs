mod production;

pub mod calyx_rs {
    use std::collections::HashMap;

    pub struct Options {
        strict: bool,
        random: rand::Rng
    }

    pub struct Grammar {
        rules: HashMap<String, String>,
    }

    pub enum Err {
        UndefinedRule { rule_name: String },
        UndefinedFilter { filter_name: String },
    }

    struct Registry {

    }

    struct Rule {
        term: String
    }

    impl Grammar {
        pub fn start(&self, start_name: &str) -> Result<(), Err> {
            return Result::Ok(());
        }
    }
}