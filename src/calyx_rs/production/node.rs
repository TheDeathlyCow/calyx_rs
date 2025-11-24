use crate::calyx_rs::CalyxError;
use crate::calyx_rs::expansion_tree::ExpansionTree;
use crate::calyx_rs::production::Production;
use crate::calyx_rs::evaluation::EvaluationContext;

struct AtomNode {
    atom: String,
}

impl Production for AtomNode {
    fn evaluate(&self, eval_context: &mut EvaluationContext) -> Result<ExpansionTree, CalyxError> {
        Ok(ExpansionTree::new_atom(&self.atom))
    }
}

struct MemoNode {
    symbol: String,
}

impl Production for MemoNode {
    fn evaluate(&self, eval_context: &mut EvaluationContext) -> Result<ExpansionTree, CalyxError> {
        todo!()
    }
}
