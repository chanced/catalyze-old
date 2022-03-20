use std::collections::HashMap;

use crate::{Artifact, Ast, File};

pub trait Module {
    fn name(&self) -> &'static str;
    fn init(&mut self);
    fn execute(&mut self, targets: HashMap<String, File>, ast: Ast) -> Vec<Artifact>;
}
