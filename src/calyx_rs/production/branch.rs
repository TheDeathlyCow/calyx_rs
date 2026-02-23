use crate::calyx_rs::CalyxError;
use crate::calyx_rs::evaluation::EvaluationContext;
use crate::calyx_rs::expansion_tree::{ExpansionTree, ExpansionType};
use crate::calyx_rs::production::node::TemplateNode;
use crate::calyx_rs::production::{Production, ProductionBranch};
use rand::Rng;
use std::collections::HashMap;

pub(crate) struct EmptyBranch {}

impl Production for EmptyBranch {
    fn evaluate(&self, eval_context: &mut EvaluationContext) -> Result<ExpansionTree, CalyxError> {
        self.evaluate_at(0, eval_context)
    }
}

impl ProductionBranch for EmptyBranch {
    fn evaluate_at(
        self: &Self,
        _index: usize,
        _eval_context: &mut EvaluationContext,
    ) -> Result<ExpansionTree, CalyxError> {
        let exp = ExpansionTree::new_atom(String::new());
        Ok(ExpansionTree::chain(ExpansionType::EmptyBranch, exp))
    }

    fn len(&self) -> usize {
        1
    }
}

pub(crate) struct UniformBranch {
    choices: Vec<TemplateNode>,
}

impl UniformBranch {
    pub(crate) fn parse(raw: &Vec<String>) -> Result<Self, CalyxError> {
        let mut choices: Vec<TemplateNode> = Vec::new();

        for term in raw {
            let template_node = TemplateNode::parse(term)?;
            choices.push(template_node)
        }

        Ok(Self { choices })
    }
}

impl Production for UniformBranch {
    fn evaluate(&self, eval_context: &mut EvaluationContext) -> Result<ExpansionTree, CalyxError> {
        let index = eval_context
            .options()
            .random_source
            .random_range(0..self.choices.len());

        self.evaluate_at(index, eval_context)
    }
}

impl ProductionBranch for UniformBranch {
    fn evaluate_at(
        &self,
        index: usize,
        eval_context: &mut EvaluationContext,
    ) -> Result<ExpansionTree, CalyxError> {
        let item = self
            .choices
            .get(index)
            .ok_or(CalyxError::ExpandedEmptyBranch)?;

        let tail = item.evaluate(eval_context)?;
        Ok(ExpansionTree::chain(ExpansionType::UniformBranch, tail))
    }

    fn len(&self) -> usize {
        self.choices.len()
    }
}

struct WeightedProduction {
    production: Box<dyn Production>,
    weight: f64,
}

impl WeightedProduction {
    fn new<P: Production + 'static>(weight: f64, production: P) -> Result<Self, CalyxError> {
        if WeightedBranch::is_invalid_weight(weight) {
            return Err(CalyxError::InvalidWeight { weight });
        }

        Ok(Self {
            production: Box::new(production),
            weight,
        })
    }
}

pub(crate) struct WeightedBranch {
    productions: Vec<WeightedProduction>,
    sum_of_weights: f64,
}

impl WeightedBranch {
    fn is_invalid_weight(weight: f64) -> bool {
        weight <= 0.0 || !weight.is_finite()
    }

    fn new(productions: Vec<WeightedProduction>) -> Result<Self, CalyxError> {
        let sum_of_weights: f64 = productions.iter().map(|production| production.weight).sum();

        if WeightedBranch::is_invalid_weight(sum_of_weights) {
            return Err(CalyxError::InvalidWeight {
                weight: sum_of_weights,
            });
        }

        Ok(Self {
            productions,
            sum_of_weights,
        })
    }

    fn get_random_production(&self, eval_context: &mut EvaluationContext) -> &WeightedProduction {
        let rng = eval_context.options().random_source.as_mut();
        let water_mark: f64 = rng.random::<f64>() * self.sum_of_weights;

        let mut cumulative = 0.0;

        for wp in &self.productions {
            cumulative += wp.weight;
            if water_mark < cumulative {
                return wp;
            }
        }

        // this should never happen, as the total weight should be greater than 0
        panic!("Unable to evaluate weighted production")
    }

    pub(crate) fn parse(raw: &HashMap<String, f64>) -> Result<Self, CalyxError> {
        let mut productions: Vec<WeightedProduction> = Vec::new();

        // remove the random ordering of the hashmap. 
        let mut entries: Vec<(&String, &f64)> = raw.iter().collect();
        entries.sort_by(|(name_a, _), (name_b, _)| name_a.cmp(name_b));

        for (name, weight) in entries {
            let node = TemplateNode::parse(&name)?;
            productions.push(WeightedProduction::new(*weight, node)?)
        }

        Self::new(productions)
    }
}

impl Production for WeightedBranch {
    fn evaluate(&self, eval_context: &mut EvaluationContext) -> Result<ExpansionTree, CalyxError> {
        let prod = self.get_random_production(eval_context);
        let choice = prod.production.evaluate(eval_context)?;
        Ok(ExpansionTree::chain(ExpansionType::WeightedBranch, choice))
    }
}

impl ProductionBranch for WeightedBranch {
    fn evaluate_at(
        &self,
        index: usize,
        eval_context: &mut EvaluationContext,
    ) -> Result<ExpansionTree, CalyxError> {
        self.productions
            .get(index)
            .ok_or(CalyxError::ExpandedEmptyBranch)?
            .production
            .evaluate(eval_context)
    }

    fn len(&self) -> usize {
        self.productions.len()
    }
}
