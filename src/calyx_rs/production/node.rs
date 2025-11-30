use crate::calyx_rs::CalyxError;
use crate::calyx_rs::evaluation::EvaluationContext;
use crate::calyx_rs::expansion_tree::{ExpansionTree, ExpansionType};
use crate::calyx_rs::production::Production;
use std::str::CharIndices;

struct AtomNode {
    atom: String,
}

impl Production for AtomNode {
    fn evaluate(&self, eval_context: &mut EvaluationContext) -> Result<ExpansionTree, CalyxError> {
        Ok(ExpansionTree::new_atom(&self.atom))
    }
}

struct TemplateNode {
    concat_nodes: Vec<Box<dyn Production>>,
}

impl Production for TemplateNode {
    fn evaluate(&self, eval_context: &mut EvaluationContext) -> Result<ExpansionTree, CalyxError> {
        let mut evaluated_results: Vec<ExpansionTree> = Vec::new();

        for node in &self.concat_nodes {
            let single_result = node.evaluate(eval_context)?;
            evaluated_results.push(single_result);
        }

        Ok(ExpansionTree::new(
            ExpansionType::Template,
            evaluated_results,
        ))
    }
}

impl TemplateNode {
    pub(crate) fn parse(raw: &String) -> Result<TemplateNode, CalyxError> {
        todo!()
    }

    fn fragment_string(raw: &str) -> Result<Vec<String>, CalyxError> {
        let mut fragments = Vec::new();
        let mut current = String::new();

        for ch in raw.chars() {
            match ch {
                '{' => {
                    if !current.is_empty() {
                        fragments.push(std::mem::take(&mut current));
                    }
                    current.push(ch);
                }
                '}' => {
                    current.push(ch);
                    fragments.push(std::mem::take(&mut current));
                }
                _ => current.push(ch),
            }
        }

        if !current.is_empty() {
            fragments.push(current);
        }

        Ok(fragments)
    }
}

#[cfg(test)]
mod tests {
    use crate::calyx_rs::production::node::TemplateNode;

    #[test]
    fn frag_with_no_delimiters() {
        let frags = TemplateNode::fragment_string("One Two Three");
        assert_eq!(vec!["One Two Three"], frags.unwrap())
    }

    #[test]
    fn frag_is_just_expansion() {
        let frags = TemplateNode::fragment_string("{One Two Three}");
        assert_eq!(vec!["{One Two Three}"], frags.unwrap())
    }

    #[test]
    fn two_adjacent_expansions() {
        let frags = TemplateNode::fragment_string("{One}{Two}");
        assert_eq!(vec!["{One}", "{Two}"], frags.unwrap())
    }

    #[test]
    fn frag_with_single_expansion() {
        let frags = TemplateNode::fragment_string("{One} Two Three");
        assert_eq!(vec!["{One}", " Two Three"], frags.unwrap())
    }

    #[test]
    fn frag_starts_with_expansion() {
        let frags = TemplateNode::fragment_string("{One} Two");
        assert_eq!(vec!["{One}", " Two"], frags.unwrap())
    }

    #[test]
    fn frag_ends_with_expansion() {
        let frags = TemplateNode::fragment_string("One {Two}");
        assert_eq!(vec!["One ", "{Two}"], frags.unwrap())
    }

    #[test]
    fn frag_with_multiple_expansion() {
        let frags = TemplateNode::fragment_string("{One} Two {Three} Four");
        assert_eq!(vec!["{One}", " Two ", "{Three}", " Four"], frags.unwrap())
    }
}
