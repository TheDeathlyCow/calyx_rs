use crate::calyx_rs::expansion_tree::ExpansionTree;
use crate::calyx_rs::production::ProductionBranch;
use crate::calyx_rs::production::branch::EmptyBranch;
use crate::calyx_rs::{CalyxError, Options};
use std::collections::HashMap;

pub struct Registry {
    rules: HashMap<String, Rule>,
    // this will always be an empty branch but is stored in the struct so that the lifetime matches
    empty_rule: Rule,
}

impl Registry {
    pub fn new(rules: HashMap<String, Rule>) -> Self {
        Self {
            rules,
            empty_rule: Rule::empty(),
        }
    }

    pub fn expand(&self, symbol: &String, options: &Options) -> Result<&Rule, CalyxError> {
        if self.rules.contains_key(symbol) {
            return self
                .rules
                .get(symbol)
                .ok_or_else(|| CalyxError::UndefinedRule {
                    rule_name: symbol.clone(),
                });
        }

        if options.strict {
            Err(CalyxError::UndefinedRule {
                rule_name: symbol.clone(),
            })
        } else {
            Ok(&self.empty_rule)
        }
    }
}

pub struct Rule {
    production: Box<dyn ProductionBranch>,
}

impl Rule {
    fn empty() -> Rule {
        Rule {
            production: Box::new(EmptyBranch {}),
        }
    }
}

pub struct EvaluationContext<'a> {
    registry: &'a Registry,
    options: &'a mut Options,
    memoized_expansions: HashMap<String, ExpansionTree>,
}

impl<'a> EvaluationContext<'a> {
    pub fn memoize_expansion(&mut self, symbol: &String) -> Result<ExpansionTree, CalyxError> {
        if !self.memoized_expansions.contains_key(symbol) {
            let expanded_tree = self.expand_symbol_once(symbol)?;

            self.memoized_expansions
                .insert(symbol.clone(), expanded_tree);
        }

        Ok(self.memoized_expansions[symbol].clone())
    }

    fn expand_symbol_once(&self, symbol: &String) -> Result<ExpansionTree, CalyxError> {
        let rule = self.registry.expand(symbol, self.options)?;

        todo!()
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
