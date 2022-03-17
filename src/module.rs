use std::{collections::HashMap, error, marker::PhantomData, rc::Rc};

use crate::{Artifact, Ast, File, Util};

pub trait Module<'a> {
    fn name(&self) -> &'static str;
    fn init(&mut self);
    fn util<U: Util>(&self) -> Rc<U>;
    fn execute<U: Util>(
        &mut self,
        targets: HashMap<String, File<'a, U>>,
        ast: Ast<'a, U>,
    ) -> Vec<Artifact>;
}

pub(crate) struct ModuleContanier<'a> {
    pub(crate) module: Box<dyn Module<'a>>,
}
impl<'a> ModuleContanier<'a>
where
    M: Module<'a>,
{
    pub fn new(module: M) -> Self {
        Self {
            module,
            _marker: PhantomData,
        }
    }
}
