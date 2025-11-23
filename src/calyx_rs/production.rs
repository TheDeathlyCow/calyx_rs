mod branch;
mod node;

use crate::calyx_rs::registry::EvaluationContext;
use crate::calyx_rs::CalyxError;

pub trait Production {
    fn evaluate(&self, eval_context: &mut EvaluationContext) -> Result<Expansion, CalyxError>;
}

pub trait ProductionBranch: Production {
    fn evaluate_at(
        &self,
        index: isize,
        eval_context: &mut EvaluationContext,
    ) -> Result<Expansion, CalyxError>;

    fn len(&self) -> usize;
}

impl<B: ProductionBranch> Production for B {
    fn evaluate(
        self: &Self,
        eval_context: &mut EvaluationContext,
    ) -> Result<Expansion, CalyxError> {
        self.evaluate_at(0, eval_context)
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

    pub fn new_atom(term: &str) -> Self {
        Expansion {
            tail: vec![],
            symbol: ExpansionType::Atom(term.to_string()),
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
