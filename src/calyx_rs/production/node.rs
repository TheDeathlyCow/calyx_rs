use crate::calyx_rs::CalyxError;
use crate::calyx_rs::production::{Expansion, Production};
use crate::calyx_rs::registry::EvaluationContext;

struct AtomNode {
    atom: String
}

impl AtomNode {
    pub(crate) fn atom(&self) -> &String {
        &self.atom
    }
}

impl Production for AtomNode {
    fn evaluate(&self, eval_context: &mut EvaluationContext) -> Result<Expansion, CalyxError> {
        Ok(Expansion::new_atom(&self.atom))
    }
}

