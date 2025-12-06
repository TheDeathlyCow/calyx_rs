pub(super) mod branch;
mod node;

use crate::calyx_rs::CalyxError;
use crate::calyx_rs::expansion_tree::ExpansionTree;
use crate::calyx_rs::evaluation::EvaluationContext;

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
