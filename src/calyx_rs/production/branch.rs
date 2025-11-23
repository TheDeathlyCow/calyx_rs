use crate::calyx_rs::CalyxError;
use crate::calyx_rs::production::{Expansion, ExpansionType, Production, ProductionBranch};
use crate::calyx_rs::registry::EvaluationContext;
use rand::seq::IndexedRandom;

struct EmptyBranch {}

impl ProductionBranch for EmptyBranch {
    fn evaluate_at(
        self: &Self,
        index: isize,
        eval_context: &mut EvaluationContext,
    ) -> Result<Expansion, CalyxError> {
        let exp = Expansion::new_atom("");
        Ok(Expansion::chain(ExpansionType::EmptyBranch, exp))
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
    ) -> Result<Expansion, CalyxError> {
        let options = eval_context.options();

        let item = self
            .choices
            .choose(&mut options.random_source)
            .ok_or(CalyxError::ExpandedEmptyBranch)?;

        let t = item.evaluate(eval_context)?;
        Ok(Expansion::chain(ExpansionType::UniformBranch, t))
    }

    fn len(&self) -> usize {
        self.choices.len()
    }
}
