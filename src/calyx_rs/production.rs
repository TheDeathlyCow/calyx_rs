pub(crate) mod production {
    use crate::calyx_rs::calyx_rs::Grammar;

    pub trait Production {
        fn evaluate(self: &Self, grammar: &Grammar) -> Option<Expansion>;
    }

    pub trait ProductionBranch: Production {
        fn evaluate_at(self: &Self, index: isize, grammar: &Grammar) -> Option<Expansion>;

        fn evaluate(self: &Self, grammar: &Grammar) -> Option<Expansion> {
            self.evaluate_at(0, grammar)
        }
    }

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

    pub struct Expansion {
        tail: Vec<Expansion>,
        symbol: ExpansionType,
    }

    impl Expansion {
        pub fn new(symbol: ExpansionType, tail: Vec<Expansion>) -> Self {
            Expansion { tail, symbol }
        }

        pub fn chain(symbol: ExpansionType, tail: Expansion) -> Self {
            Expansion {
                tail: vec![tail],
                symbol,
            }
        }

        pub fn new_atom(term: String) -> Self {
            Expansion {
                tail: vec![],
                symbol: ExpansionType::Atom(term),
            }
        }

        pub fn flatten(&self) -> String {
            let mut term = String::from("");
            self.collect_atoms(&mut term);
            term
        }

        fn collect_atoms(&self, concat: &mut String) {
            match &self.symbol {
                ExpansionType::Atom(term) => {
                    concat.push_str(term.as_str());
                }
                _ => {
                    for exp in &self.tail {
                        exp.collect_atoms(concat);
                    }
                }
            }
        }
    }
}
