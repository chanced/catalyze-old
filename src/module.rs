use std::collections::HashMap;

use crate::{Artifact, Ast, File};

pub trait Module<'a> {
    fn name(&self) -> &'static str;
    fn init(&mut self);
    fn execute(&mut self, targets: HashMap<String, File<'a>>, ast: Ast<'a>) -> Vec<Artifact>;
}
