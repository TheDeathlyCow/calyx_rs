pub(crate) mod production {
    use crate::calyx_rs::calyx_rs::Grammar;

    pub(crate) trait Production {
        fn evaluate(self: &Self, grammar: &Grammar) -> Option<Expansion>;
    }

    pub(crate) trait ProductionBranch: Production {
        fn evaluate_at(self: &Self, index: isize, grammar: &Grammar) -> Option<Expansion>;

        fn evaluate(self: &Self, grammar: &Grammar) -> Option<Expansion> {
            self.evaluate_at(0, grammar)
        }
    }

    enum ExpansionType {
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

    pub(crate) struct Expansion {
        tail: Vec<Expansion>,
        symbol: ExpansionType,
    }

    impl Expansion {
        pub fn flatten(&self) -> String {
            let mut term = String::from("");
            self.collectAtoms(&mut term);
            term
        }

        fn collectAtoms(&self, concat: &mut String) {
            match &self.symbol {
                ExpansionType::Atom(term) => {
                    concat.push_str(term.as_str());
                }
                _ => {
                    for exp in &self.tail {
                        exp.collectAtoms(concat);
                    }
                }
            }
        }
    }
}
