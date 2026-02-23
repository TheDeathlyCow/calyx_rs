use crate::generation::CalyxError;
use crate::generation::evaluation::EvaluationContext;
use crate::generation::expansion_tree::{ExpansionTree, ExpansionType};
use crate::generation::production::Production;

struct AtomNode {
    atom: String,
}

impl Production for AtomNode {
    fn evaluate(&self, _eval_context: &mut EvaluationContext) -> Result<ExpansionTree, CalyxError> {
        Ok(ExpansionTree::new_atom(self.atom.clone()))
    }
}

struct ExpressionNode {
    reference: String,
}

impl Production for ExpressionNode {
    fn evaluate(&self, eval_context: &mut EvaluationContext) -> Result<ExpansionTree, CalyxError> {
        let result = eval_context.expand_and_evaluate(&self.reference)?;
        Ok(ExpansionTree::chain(ExpansionType::Expression, result))
    }
}

struct ExpressionChain {
    expression_rule: Box<dyn Production>,
    filter_names: Vec<String>,
}

impl Production for ExpressionChain {
    fn evaluate(&self, eval_context: &mut EvaluationContext) -> Result<ExpansionTree, CalyxError> {
        let mut initial_string: String = self.expression_rule.evaluate(eval_context)?.flatten();

        for filter_name in &self.filter_names {
            let filter = eval_context
                .registry()
                .get_filter(filter_name)
                .ok_or_else(|| CalyxError::UndefinedFilter {
                    filter_name: filter_name.clone(),
                })?;

            filter(&mut initial_string);
        }

        Ok(ExpansionTree::chain(
            ExpansionType::ExpressionChain,
            ExpansionTree::new_atom(initial_string),
        ))
    }
}

struct MemoNode {
    symbol: String,
}

impl Production for MemoNode {
    fn evaluate(&self, eval_context: &mut EvaluationContext) -> Result<ExpansionTree, CalyxError> {
        let tree = eval_context.memoize_expansion(&self.symbol)?;

        Ok(ExpansionTree::chain(ExpansionType::Memo, tree))
    }
}

struct UniqueNode {
    symbol: String,
}

impl Production for UniqueNode {
    fn evaluate(&self, eval_context: &mut EvaluationContext) -> Result<ExpansionTree, CalyxError> {
        let tree = eval_context.unique_expansion(&self.symbol)?;

        Ok(ExpansionTree::chain(ExpansionType::Unique, tree))
    }
}

#[cfg(test)]
mod unique_tests {
    use crate::generation::{Grammar, Options};
    use rand::SeedableRng;
    use rand::prelude::StdRng;

    #[test]
    fn unique_node_cycles_through_each_template_in_branch() {
        let rng = StdRng::seed_from_u64(12345);
        let mut grammar = Grammar::from_options(Options::new(true, rng));

        assert!(
            grammar
                .start_single(String::from("{$medal} {$medal} {$medal}"))
                .is_ok()
        );

        assert!(
            grammar
                .uniform_rule(
                    String::from("medal"),
                    &vec![
                        String::from("gold"),
                        String::from("silver"),
                        String::from("bronze"),
                    ],
                )
                .is_ok()
        );

        let result = grammar.generate().expect("Error during generation");
        let text = result.flatten();

        assert_eq!("bronze silver gold", text);
    }
}

pub(crate) struct TemplateNode {
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
        let fragments = Self::fragment_string(raw);

        let mut concat_nodes: Vec<Box<dyn Production>> = Vec::new();

        for fragment in fragments {
            if fragment.is_empty() {
                continue;
            }

            if fragment.starts_with("{") && fragment.ends_with("}") {
                let raw_expression: String = fragment
                    .get(1..fragment.len() - 1)
                    .ok_or_else(|| CalyxError::InvalidExpression {
                        expression: fragment.clone(),
                    })?
                    .to_string();

                let expression = Self::parse_expression(raw_expression)?;
                concat_nodes.push(expression)
            } else {
                concat_nodes.push(Box::new(AtomNode { atom: fragment }))
            }
        }

        Ok(TemplateNode { concat_nodes })
    }

    fn parse_expression(raw_expression: String) -> Result<Box<dyn Production>, CalyxError> {
        let components = raw_expression
            .split(".")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        if components.len() < 2 {
            Self::parse_simple_expression(raw_expression)
        } else {
            Self::parse_expression_chain(components)
        }
    }

    fn parse_simple_expression(raw_expression: String) -> Result<Box<dyn Production>, CalyxError> {
        let sigil = raw_expression
            .chars()
            .nth(0)
            .ok_or_else(|| CalyxError::InvalidExpression {
                expression: raw_expression.clone(),
            })?;

        let mut raw_expression = raw_expression;

        match sigil {
            '@' => {
                raw_expression.remove(0);
                Ok(Box::new(MemoNode {
                    symbol: raw_expression,
                }))
            }
            '$' => {
                raw_expression.remove(0);
                Ok(Box::new(UniqueNode {
                    symbol: raw_expression,
                }))
            }
            _ => Ok(Box::new(ExpressionNode {
                reference: raw_expression,
            })),
        }
    }

    fn parse_expression_chain(
        mut raw_chain: Vec<String>,
    ) -> Result<Box<dyn Production>, CalyxError> {
        let expression_name = raw_chain.remove(0);
        let expression_rule = Self::parse_simple_expression(expression_name)?;

        Ok(Box::new(ExpressionChain {
            expression_rule,
            filter_names: raw_chain,
        }))
    }

    fn fragment_string(raw: &str) -> Vec<String> {
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

        fragments
    }
}

#[cfg(test)]
mod template_tests {
    use crate::generation::production::node::TemplateNode;

    #[test]
    fn frag_with_no_delimiters() {
        let frags = TemplateNode::fragment_string("One Two Three");
        assert_eq!(vec!["One Two Three"], frags)
    }

    #[test]
    fn frag_is_just_expansion() {
        let frags = TemplateNode::fragment_string("{One Two Three}");
        assert_eq!(vec!["{One Two Three}"], frags)
    }

    #[test]
    fn two_adjacent_expansions() {
        let frags = TemplateNode::fragment_string("{One}{Two}");
        assert_eq!(vec!["{One}", "{Two}"], frags)
    }

    #[test]
    fn frag_with_single_expansion() {
        let frags = TemplateNode::fragment_string("{One} Two Three");
        assert_eq!(vec!["{One}", " Two Three"], frags)
    }

    #[test]
    fn frag_starts_with_expansion() {
        let frags = TemplateNode::fragment_string("{One} Two");
        assert_eq!(vec!["{One}", " Two"], frags)
    }

    #[test]
    fn frag_ends_with_expansion() {
        let frags = TemplateNode::fragment_string("One {Two}");
        assert_eq!(vec!["One ", "{Two}"], frags)
    }

    #[test]
    fn frag_with_multiple_expansion() {
        let frags = TemplateNode::fragment_string("{One} Two {Three} Four");
        assert_eq!(vec!["{One}", " Two ", "{Three}", " Four"], frags)
    }
}
