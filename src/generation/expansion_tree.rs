#[derive(Clone)]
#[derive(Debug)]
pub enum ExpansionType {
    Atom(String),
    Result,
    UniformBranch,
    WeightedBranch,
    EmptyBranch,
    AffixTable,
    Template,
    Expression,
    ExpressionChain,
    Memo,
    Unique,
}

#[derive(Clone)]
#[derive(Debug)]
pub struct ExpansionTree {
    children: Vec<ExpansionTree>,
    symbol: ExpansionType,
}

impl ExpansionTree {
    pub fn children(&self) -> &Vec<ExpansionTree> {
        &self.children
    }

    pub fn symbol(&self) -> ExpansionType {
        self.symbol.clone()
    }

    pub fn new(symbol: ExpansionType, tail: Vec<ExpansionTree>) -> Self {
        ExpansionTree { children: tail, symbol }
    }

    pub fn chain(symbol: ExpansionType, tail: ExpansionTree) -> Self {
        ExpansionTree {
            children: vec![tail],
            symbol,
        }
    }

    pub fn new_atom(term: String) -> Self {
        ExpansionTree {
            children: vec![],
            symbol: ExpansionType::Atom(term.to_string()),
        }
    }

    pub fn flatten(&self) -> String {
        let mut term = String::from("");
        self.collect_atoms(&mut term);
        term
    }

    fn collect_atoms(&self, concat: &mut String) {
        if let ExpansionType::Atom(term) = &self.symbol {
            concat.push_str(term.as_str());
        } else {
            for exp in &self.children {
                exp.collect_atoms(concat);
            }
        }
    }
}

#[cfg(test)]
mod expansion_tree_tests {
    use crate::generation::expansion_tree::{ExpansionTree, ExpansionType};

    #[test]
    fn flatten_expansion_tree_to_atoms() {
        let tail = vec![
            ExpansionTree::new_atom(String::from("-ONE-")),
            ExpansionTree::new_atom(String::from("-TWO-")),
            ExpansionTree::new_atom(String::from("-THREE-")),
        ];

        let exp = ExpansionTree::new(ExpansionType::Template, tail);

        let text = exp.flatten();
        assert_eq!(text, "-ONE--TWO--THREE-");
    }
}