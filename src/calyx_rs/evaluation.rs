use crate::calyx_rs::expansion_tree::ExpansionTree;
use crate::calyx_rs::filter::Filter;
use crate::calyx_rs::production::branch::EmptyBranch;
use crate::calyx_rs::production::branch::UniformBranch;
use crate::calyx_rs::production::ProductionBranch;
use crate::calyx_rs::{CalyxError, Grammar, Options};
use rand::seq::SliceRandom;
use std::collections::HashMap;

pub(crate) struct Registry {
    rules: HashMap<String, Box<dyn ProductionBranch>>,
    filters: HashMap<String, Box<dyn Filter>>,
    // this will always be an empty branch but is stored in the struct so that the lifetime matches
    empty_rule: EmptyBranch,
}

impl Registry {
    pub(crate) fn new() -> Self {
        Self {
            rules: HashMap::new(),
            filters: HashMap::new(),
            empty_rule: EmptyBranch {},
        }
    }

    pub(crate) fn define_rule(
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

    pub(crate) fn expand(
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

    pub(crate) fn get_filter(&self, filter_name: &String) -> Option<&dyn Filter> {
        self.filters.get(filter_name).map(|v| v.as_ref())
    }
}

pub(crate) struct EvaluationContext<'a> {
    registry: &'a Registry,
    options: &'a mut Options,
    memoized_expansions: HashMap<String, ExpansionTree>,
    cycles: HashMap<String, UniqueCycle>,
}

impl<'a> EvaluationContext<'a> {
    pub(crate) fn new(grammar: &'a mut Grammar) -> EvaluationContext<'a> {
        EvaluationContext {
            registry: &grammar.registry,
            options: &mut grammar.options,
            memoized_expansions: HashMap::new(),
            cycles: HashMap::new(),
        }
    }

    pub(crate) fn unique_expansion(
        &mut self,
        symbol: &String,
    ) -> Result<ExpansionTree, CalyxError> {
        let rule = self.registry.expand(symbol, self.options)?;

        if !self.cycles.contains_key(symbol) {
            let cycle = UniqueCycle::new(rule.len());

            self.cycles.insert(symbol.clone(), cycle);
        }

        let cycle = self
            .cycles
            .get_mut(symbol)
            .ok_or_else(|| CalyxError::UndefinedRule {
                rule_name: symbol.clone(),
            })?;

        let index = cycle.poll(self.options);

        rule.evaluate_at(index, self)
    }

    pub(crate) fn memoize_expansion(
        &mut self,
        symbol: &String,
    ) -> Result<ExpansionTree, CalyxError> {
        if !self.memoized_expansions.contains_key(symbol) {
            let expanded_tree = self.expand_and_evaluate(symbol)?;

            self.memoized_expansions
                .insert(symbol.clone(), expanded_tree);
        }

        let tree =
            self.memoized_expansions
                .get(symbol)
                .ok_or_else(|| CalyxError::UndefinedRule {
                    rule_name: symbol.clone(),
                })?;

        Ok(tree.clone())
    }

    pub(crate) fn expand_and_evaluate(
        &mut self,
        symbol: &String,
    ) -> Result<ExpansionTree, CalyxError> {
        let rule = self.registry.expand(symbol, self.options)?;
        rule.evaluate(self)
    }
}

impl<'a> EvaluationContext<'a> {
    pub(crate) fn registry(&self) -> &Registry {
        self.registry
    }

    pub(crate) fn options(&mut self) -> &mut Options {
        self.options
    }
}

struct UniqueCycle {
    count: usize,
    index: usize,
    sequence: Vec<usize>,
}

impl UniqueCycle {
    fn new(count: usize) -> UniqueCycle {
        let sequence = Vec::with_capacity(count);
        UniqueCycle {
            count,
            index: count - 1,
            sequence,
        }
    }

    fn populate_sequence(&mut self) {
        self.sequence.clear();
        for i in 0..self.count {
            self.sequence.push(i);
        }
    }

    fn shuffle(&mut self, options: &mut Options) {
        self.populate_sequence();
        let rng = &mut *options.random_source;
        self.sequence.as_mut_slice().shuffle(rng);
    }

    fn poll(&mut self, options: &mut Options) -> usize {
        self.index += 1;

        if self.index >= self.count {
            self.shuffle(options);
            self.index = 0;
        }

        self.sequence.get(self.index).copied().unwrap_or(0)
    }
}
