use std::collections::HashMap;
use crate::calyx_rs::production::ProductionBranch;

pub struct Registry {
    context: HashMap<String, Rule>,
}

pub struct EvaluationContext {
    context: HashMap<String, Rule>,
}

pub struct Rule {
    production: Box<dyn ProductionBranch>
}