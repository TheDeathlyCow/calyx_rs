use crate::calyx_rs::production::ProductionBranch;
use crate::calyx_rs::Options;
use std::collections::HashMap;

pub struct Registry {
    context: HashMap<String, Rule>,
}

pub struct EvaluationContext<'a> {
    registry: &'a Registry,
    options: &'a mut Options,
    context: HashMap<String, Rule>,
}

pub struct Rule {
    production: Box<dyn ProductionBranch>,
}

impl<'a> EvaluationContext<'a> {
    pub(crate) fn registry(&self) -> &Registry {
        self.registry
    }

    pub(crate) fn options(&mut self) -> &mut Options {
        self.options
    }

    pub(crate) fn context(&self) -> &HashMap<String, Rule> {
        &self.context
    }
}
