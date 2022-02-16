use crate::new_extension_list;
use crate::ExtensionList;
use crate::Lang;
use crate::Node;
use crate::{File, Package};
use prost_types::compiler::CodeGeneratorRequest;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

type TargetFiles<L> = Rc<RefCell<HashMap<String, Rc<File<L>>>>>;
type Packages<L> = Rc<RefCell<HashMap<String, Rc<Package<L>>>>>;

type Nodes<L> = Rc<RefCell<HashMap<String, Node<L>>>>;
#[derive(Debug)]
pub struct Ast<L> {
    targets: TargetFiles<L>,
    packages: Packages<L>,
    nodes: Nodes<L>,
    extensions: ExtensionList<L>,
    pub lang: L,
}

// Ast encapsulates the entirety of the input CodeGeneratorRequest from protoc,
// parsed to build the Node graph used by catalyze.
impl<L: Lang> Ast<L> {
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

fn create_map<K, V>(size: Option<usize>) -> Rc<RefCell<HashMap<K, V>>> {
    match size {
        Some(size) => Rc::new(RefCell::new(HashMap::with_capacity(size))),
        None => Rc::new(RefCell::new(HashMap::default())),
    }
}
// process_code_generator_request
pub fn process_code_generator_request<L: Lang>(request: CodeGeneratorRequest, lang: L) -> Ast<L> {
    let mut ast = Ast {
        lang,
        targets: create_map(Some(request.proto_file.len())),
        packages: create_map(None),
        nodes: create_map(None),
        extensions: new_extension_list(),
    };

    let target_list = request
        .file_to_generate
        .iter()
        .cloned()
        .collect::<HashSet<String>>();

    for file in request.proto_file {
        let pkg = file.package;
    }

    ast
}

enum Source {
    CodeGeneratorRequest(CodeGeneratorRequest),
}
