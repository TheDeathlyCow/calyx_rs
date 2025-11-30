use crate::calyx_rs::CalyxError;
use crate::calyx_rs::expansion_tree::{ExpansionTree, ExpansionType};
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

struct TemplateNode {
    concat_nodes: Vec<Box<dyn Production>>
}

impl TemplateNode {
    pub(crate) fn parse(raw: &String) -> Result<TemplateNode, CalyxError> {
        todo!()
    }
}

impl Production for TemplateNode {
    fn evaluate(&self, eval_context: &mut EvaluationContext) -> Result<ExpansionTree, CalyxError> {
        let mut evaluated_results: Vec<ExpansionTree> = Vec::new();

        for node in &self.concat_nodes {
            let single_result = node.evaluate(eval_context)?;
            evaluated_results.push(single_result);
        }

        Ok(ExpansionTree::new(ExpansionType::Template, evaluated_results))
    }
}