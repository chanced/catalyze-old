use crate::Node;
use crate::{lang::Lang, File, Package};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
#[derive(Debug)]
pub struct Ast<L: Lang> {
    targets: RefCell<HashMap<String, Rc<File<L>>>>,
    packages: RefCell<HashMap<String, Rc<Package<L>>>>,
    nodes: RefCell<HashMap<String, Node<L>>>,
}
// Ast encapsulates the entirety of the input CodeGeneratorRequest from protoc,
// parsed to build the Node graph used by catalyze.
impl<L: Lang> Ast<L> {
    pub(crate) fn new(request: &prost_types::FileDescriptorSet) -> Self {
        let mut targets = HashMap::new();
        let mut packages = HashMap::new();
        let mut nodes = HashMap::new();

        Self {
            targets: RefCell::new(targets),
            packages: RefCell::new(packages),
            nodes: RefCell::new(nodes),
        }
    }

    // targets returns a hashmap of the files specified in the protoc execution.
    pub fn targets(&self) -> HashMap<String, Rc<File<L>>> {
        self.targets.borrow().clone()
    }
    // packages returns all the imported packages (including those for the target
    // Files). This is limited to just the files that were imported by target
    // protos, either directly or transitively.
    pub fn packages(&self) -> HashMap<String, Rc<Package<L>>> {
        self.packages.borrow().clone()
    }
    pub fn package(&self, name: &str) -> Option<Rc<Package<L>>> {
        self.packages.borrow().get(name).cloned()
    }
    pub fn file(&self, name: &str) -> Option<Rc<File<L>>> {
        self.targets.borrow().get(name).cloned()
    }
    // node allows getting a Node from the graph by its fully-qualified name
    // (FQN). The FQN uses dot notation of the form ".{package}.{node}", or the
    // input path for files.
    pub fn node(&self, name: &str) -> Option<Node<L>> {
        self.nodes.borrow().get(name).cloned()
    }
}
