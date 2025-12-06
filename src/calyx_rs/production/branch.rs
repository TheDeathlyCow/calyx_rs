use crate::calyx_rs::evaluation::EvaluationContext;
use crate::calyx_rs::expansion_tree::ExpansionType::Template;
use crate::calyx_rs::expansion_tree::{ExpansionTree, ExpansionType};
use crate::calyx_rs::production::node::TemplateNode;
use crate::calyx_rs::production::{Production, ProductionBranch};
use crate::calyx_rs::{CalyxError, evaluation};
use rand::seq::IndexedRandom;

pub(crate) struct EmptyBranch {}

impl Production for EmptyBranch {
    fn evaluate(&self, eval_context: &mut EvaluationContext) -> Result<ExpansionTree, CalyxError> {
        self.evaluate_at(0, eval_context)
    }
}

impl ProductionBranch for EmptyBranch {
    fn evaluate_at(
        self: &Self,
        index: usize,
        eval_context: &mut EvaluationContext,
    ) -> Result<ExpansionTree, CalyxError> {
        let exp = ExpansionTree::new_atom("");
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
    pub(crate) fn parse(raw: &Vec<String>) -> Result<UniformBranch, CalyxError> {
        let mut choices: Vec<TemplateNode> = Vec::new();

        for term in raw {
            let template_node = TemplateNode::parse(term)?;
            choices.push(template_node)
        }

        Ok(UniformBranch { choices })
    }
}

impl Production for UniformBranch {
    fn evaluate(&self, eval_context: &mut EvaluationContext) -> Result<ExpansionTree, CalyxError> {
        todo!()
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
