use crate::calyx_rs::CalyxError;
use crate::calyx_rs::production::{Production, ProductionBranch};
use crate::calyx_rs::evaluation::EvaluationContext;
use rand::seq::IndexedRandom;
use crate::calyx_rs::expansion_tree::{ExpansionTree, ExpansionType};

struct EmptyBranch {}

impl ProductionBranch for EmptyBranch {
    fn evaluate_at(
        self: &Self,
        index: isize,
        eval_context: &mut EvaluationContext,
    ) -> Result<ExpansionTree, CalyxError> {
        let exp = ExpansionTree::new_atom("");
        Ok(ExpansionTree::chain(ExpansionType::EmptyBranch, exp))
    }

    fn len(&self) -> usize {
        1
    }
}

struct UniformBranch {
    choices: Vec<Box<dyn Production>>,
}

impl ProductionBranch for UniformBranch {
    fn evaluate_at(
        &self,
        index: isize,
        eval_context: &mut EvaluationContext,
    ) -> Result<ExpansionTree, CalyxError> {
        let options = eval_context.options();

        let item = self
            .choices
            .choose(&mut options.random_source)
            .ok_or(CalyxError::ExpandedEmptyBranch)?;

        let tail = item.evaluate(eval_context)?;
        Ok(ExpansionTree::chain(ExpansionType::UniformBranch, tail))
    }

    fn len(&self) -> usize {
        self.choices.len()
    }
}
