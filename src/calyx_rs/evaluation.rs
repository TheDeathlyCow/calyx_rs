use crate::calyx_rs::expansion_tree::ExpansionTree;
use crate::calyx_rs::production::ProductionBranch;
use crate::calyx_rs::production::branch::EmptyBranch;
use crate::calyx_rs::production::branch::UniformBranch;
use crate::calyx_rs::{CalyxError, Options};
use std::collections::HashMap;

pub struct Registry {
    rules: HashMap<String, Box<dyn ProductionBranch>>,
    // this will always be an empty branch but is stored in the struct so that the lifetime matches
    empty_rule: EmptyBranch,
}

impl Registry {
    pub fn new(rules: HashMap<String, Box<dyn ProductionBranch>>) -> Self {
        Self {
            rules,
            empty_rule: EmptyBranch {},
        }
    }

    pub fn define_rule(
        &mut self,
        symbol: &str,
        production: &Vec<String>,
    ) -> Result<(), CalyxError> {
        let symbol = symbol.to_string();
        
        if self.rules.contains_key(&symbol) {
            return Err(CalyxError::DuplicateRule { rule_name: symbol });
        }

        let branch = UniformBranch::parse(production)?;
        self.rules.insert(symbol, Box::new(branch));

        Ok(())
    }

    pub fn expand(
        &self,
        symbol: &String,
        options: &Options,
    ) -> Result<&dyn ProductionBranch, CalyxError> {
        let stored_rule = self.rules.get(symbol);

        match stored_rule {
            Some(rule) => Ok(rule.as_ref()),
            None => {
                if options.strict {
                    Err(CalyxError::UndefinedRule {
                        rule_name: symbol.clone(),
                    })
                } else {
                    Ok(&self.empty_rule)
                }
            }
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
            let expanded_tree = self.expand_and_evaluate(symbol)?;

            self.memoized_expansions
                .insert(symbol.clone(), expanded_tree);
        }

        Ok(self.memoized_expansions[symbol].clone())
    }

    pub fn expand_and_evaluate(&mut self, symbol: &String) -> Result<ExpansionTree, CalyxError> {
        let rule = self.registry.expand(symbol, self.options)?;
        rule.evaluate(self)
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
