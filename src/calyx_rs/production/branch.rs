use crate::calyx_rs::Grammar;
use crate::calyx_rs::production::{Expansion, ExpansionType, Production, ProductionBranch};

struct EmptyBranch {}

impl ProductionBranch for EmptyBranch {
    fn evaluate_at(self: &Self, index: isize, grammar: &Grammar) -> Option<Expansion> {
        let exp = Expansion::new_atom("");
        Some(Expansion::chain(ExpansionType::EmptyBranch, exp))
    }

    fn length(&self) -> isize {
        1
    }
}