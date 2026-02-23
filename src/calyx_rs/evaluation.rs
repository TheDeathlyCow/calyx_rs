use crate::calyx_rs::expansion_tree::ExpansionTree;
use crate::calyx_rs::filter::{Filter, create_builtin_filters};
use crate::calyx_rs::production::ProductionBranch;
use crate::calyx_rs::production::branch::{EmptyBranch, WeightedBranch};
use crate::calyx_rs::production::branch::UniformBranch;
use crate::calyx_rs::{CalyxError, Grammar, Options};
use rand::seq::SliceRandom;
use std::collections::HashMap;

pub(crate) struct Registry {
    rules: HashMap<String, Box<dyn ProductionBranch>>,
    filters: HashMap<String, Filter>,
    // this will always be an empty branch but is stored in the struct so that the lifetime matches
    empty_rule: EmptyBranch,
}

impl Registry {
    pub(crate) fn new() -> Self {
        Self {
            rules: HashMap::new(),
            filters: create_builtin_filters(),
            empty_rule: EmptyBranch {},
        }
    }

    pub(crate) fn define_rule(
        &mut self,
        symbol: String,
        production: &Vec<String>,
    ) -> Result<(), CalyxError> {
        if self.rules.contains_key(&symbol) {
            return Err(CalyxError::DuplicateRule { rule_name: symbol });
        }

        let branch = UniformBranch::parse(production)?;
        self.rules.insert(symbol, Box::new(branch));

        Ok(())
    }

    pub(crate) fn define_weighted_rule(
        &mut self,
        symbol: String,
        production: &HashMap<String, f64>,
    ) -> Result<(), CalyxError> {
        if self.rules.contains_key(&symbol) {
            return Err(CalyxError::DuplicateRule { rule_name: symbol });
        }

        let branch = WeightedBranch::parse(production)?;
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

    pub(crate) fn get_filter(&self, filter_name: &String) -> Option<&Filter> {
        self.filters.get(filter_name)
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

        let index = if count > 0 { count - 1 } else { 0 };

        UniqueCycle {
            count,
            index,
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
        if self.count == 0 {
            return 0;
        }

        self.index += 1;

        if self.index >= self.count {
            self.shuffle(options);
            self.index = 0;
        }

        self.sequence.get(self.index).copied().unwrap_or(0)
    }
}

#[cfg(test)]
mod cycle_tests {
    use rand::prelude::StdRng;
    use rand::SeedableRng;
    use crate::calyx_rs::Options;
    use crate::calyx_rs::evaluation::UniqueCycle;

    #[test]
    fn empty_cycle_always_returns_0() {
        let mut options = Options::new(false, rand::rng());
        let mut cycle = UniqueCycle::new(0);

        assert_eq!(cycle.poll(&mut options), 0);
        assert_eq!(cycle.poll(&mut options), 0);
        assert_eq!(cycle.poll(&mut options), 0);
    }

    #[test]
    fn cycle_length_one_always_returns_0() {
        let mut options = Options::new(false, rand::rng());
        let mut cycle = UniqueCycle::new(1);

        assert_eq!(cycle.poll(&mut options), 0);
        assert_eq!(cycle.poll(&mut options), 0);
        assert_eq!(cycle.poll(&mut options), 0);
    }

    #[test]
    fn cycles_refresh_when_fully_consumed() {
        let rng = StdRng::seed_from_u64(12345);
        let mut options = Options::new(false, rng);

        let count: usize = 3;
        let mut cycle = UniqueCycle::new(count);

        let mut results: Vec<usize> = Vec::new();

        for _ in 0..(2 * count) {
            results.push(cycle.poll(&mut options));
        }

        results.sort();

        assert_eq!(results, vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn cycles_are_different_each_time() {
        let rng = StdRng::seed_from_u64(12345);
        let mut options = Options::new(false, rng);

        let count: usize = 3;
        let mut cycle = UniqueCycle::new(count);

        let mut first_results: Vec<usize> = Vec::new();
        let mut second_results: Vec<usize> = Vec::new();

        for _ in 0..count {
            first_results.push(cycle.poll(&mut options));
        }

        for _ in 0..count {
            second_results.push(cycle.poll(&mut options));
        }

        assert_eq!(first_results.len(), second_results.len());
        assert_ne!(first_results, second_results);
    }
}
