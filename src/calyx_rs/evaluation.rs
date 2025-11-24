use crate::calyx_rs::Options;
use crate::calyx_rs::expansion_tree::ExpansionTree;
use crate::calyx_rs::production::ProductionBranch;
use std::collections::HashMap;

pub struct Registry {
    rules: HashMap<String, Rule>,
}

pub struct Rule {
    production: Box<dyn ProductionBranch>,
}

pub struct EvaluationContext<'a> {
    registry: &'a Registry,
    options: &'a mut Options,
    memoized_expansions: HashMap<String, ExpansionTree>,
}

impl<'a> EvaluationContext<'a> {
    pub fn memoize_expansion(&self, symbol: &String) -> ExpansionTree {
        if (!self.memoized_expansions.contains_key(symbol)) {
            let rule = self.registry;
        }

        self.memoized_expansions[symbol].clone()
    }
}

impl<'a> EvaluationContext<'a> {
    pub fn registry(&self) -> &Registry {
        self.registry
    }

    pub fn options(&mut self) -> &mut Options {
        self.options
    }

    pub fn memoized_expansions(&self) -> &HashMap<String, ExpansionTree> {
        &self.memoized_expansions
    }
}
