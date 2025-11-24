#[derive(Clone)]
pub enum ExpansionType {
    Atom(String),
    Result,
    UniformBranch,
    WeightedBranch,
    EmptyBranch,
    AffixTable,
    Template,
    Expression,
    Memo,
    Uniq,
}

#[derive(Clone)]
pub struct ExpansionTree {
    children: Vec<ExpansionTree>,
    symbol: ExpansionType,
}

impl ExpansionTree {
    pub fn new(symbol: ExpansionType, tail: Vec<ExpansionTree>) -> Self {
        ExpansionTree { children: tail, symbol }
    }

    pub fn chain(symbol: ExpansionType, tail: ExpansionTree) -> Self {
        ExpansionTree {
            children: vec![tail],
            symbol,
        }
    }

    pub fn new_atom(term: &str) -> Self {
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