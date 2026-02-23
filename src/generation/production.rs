pub(super) mod branch;
mod node;

use crate::generation::evaluation::EvaluationContext;
use crate::generation::expansion_tree::ExpansionTree;
use crate::generation::CalyxError;

pub trait Production {
    fn evaluate(&self, eval_context: &mut EvaluationContext) -> Result<ExpansionTree, CalyxError>;
}

pub trait ProductionBranch: Production {
    fn evaluate_at(
        &self,
        index: usize,
        eval_context: &mut EvaluationContext,
    ) -> Result<ExpansionTree, CalyxError>;

    fn len(&self) -> usize;
}
