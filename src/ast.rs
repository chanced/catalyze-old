use crate::Extension;
use crate::ExtensionList;
use crate::Lang;
use crate::Node;
use crate::{File, Package};
use prost_types::compiler::CodeGeneratorRequest;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug)]
pub struct Ast<U> {
    pub targets: HashMap<String, Rc<File<U>>>,
    pub packages: HashMap<String, Rc<Package<U>>>,
    pub nodes: HashMap<String, Node<U>>,
    pub extensions: Vec<Rc<Extension<U>>>,
    pub util: U,
}

// Ast encapsulates the entirety of the input CodeGeneratorRequest from protoc,
// parsed to build the Node graph used by catalyze.
impl<U> Ast<U> {
    pub fn package(&self, name: &str) -> Option<Rc<Package<U>>> {
        self.packages.get(name).cloned()
    }
    pub fn file(&self, name: &str) -> Option<Rc<File<U>>> {
        self.targets.get(name).cloned()
    }
}

impl<U> Ast<U> {
    pub fn new(util: U) -> Self {
        Self {
            util,
            targets: HashMap::new(),
            packages: HashMap::new(),
            nodes: HashMap::new(),
            extensions: Vec::new(),
        }
    }
}

impl<U: Clone> Ast<U> {
    // node allows getting a Node from the graph by its fully-qualified name
    // (FQN). The FQN uses dot notation of the form ".{package}.{node}", or the
    // input path for files.
    pub fn node(&self, name: &str) -> Option<Node<U>> {
        self.nodes.get(name).cloned()
    }
}

// process_code_generator_request
pub fn process_code_generator_request<U>(request: CodeGeneratorRequest, util: U) -> Ast<U> {
    let mut ast = Ast {
        util,
        targets: HashMap::with_capacity(request.proto_file.len()),
        packages: HashMap::default(),
        nodes: HashMap::default(),
        extensions: Vec::default(),
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

impl<U> Ast<U> {
    fn hydrate_pkg(&self, fd: &prost_types::FileDescriptorProto) -> Rc<Package<U>> {
        let lookup = fd.package();
        if self.packages.get(lookup).is_some() {
            return self.packages.get(lookup).unwrap().clone();
        }
    }
}

enum Source {
    CodeGeneratorRequest(CodeGeneratorRequest),
}
