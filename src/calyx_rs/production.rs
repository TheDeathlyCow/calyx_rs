mod production {
    use crate::calyx_rs::calyx_rs::Grammar;

    trait Production {
        fn evaluate(grammar: &Grammar) -> Option<Expansion>;
    }

    trait ProductionBranch: Production {
        fn evaluateAt(index: isize, grammar: &Grammar) -> Option<Expansion>;

        fn evalute(grammar: &Grammar) -> Option<Expansion> {
            return Self::evaluateAt(0, grammar);
        }
    }

    enum ExpansionType {
        Result,
        UniformBranch,
        WeightedBranch,
        EmptyBranch,
        AffixTable,
        Template,
        Expression,
        Atom,
        Memo,
        Uniq
    }

    struct Expansion {

    }
}