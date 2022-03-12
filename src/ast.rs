use crate::iter::Iter;
use crate::proto::FileDescriptor;
use crate::util::Util;
use crate::AllNodes;
use crate::Extensions;
use crate::Node;
use crate::Source;
use crate::Type;
use crate::{File, Package};

use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

use anyhow::anyhow;

// protoc
// --include_imports
// --include_source_info
// --proto_path=[dep dir path]
// --descriptor_set_out=[path].bin
// [path].proto

/// Ast encapsulates the entirety of the input CodeGeneratorRequest from protoc,
/// parsed to build the Node graph used by catalyze.
#[derive(Debug)]
pub(crate) struct AstDetail<'a, U> {
    files: HashMap<String, File<'a, U>>,
    file_list: Rc<RefCell<Vec<File<'a, U>>>>,
    targets: HashMap<String, File<'a, U>>,
    packages: HashMap<String, Package<'a, U>>,
    package_list: Rc<RefCell<Vec<Package<'a, U>>>>,
    defined_extensions: Extensions<'a, U>,
    nodes: HashMap<String, Node<'a, U>>,
    util: Rc<U>,
}
impl<'a, U> AstDetail<'a, U> {
    pub fn package(&self, name: &str) -> Option<Package<'a, U>> {
        self.packages.get(name).cloned()
    }
    pub fn file(&self, name: &str) -> Option<File<'a, U>> {
        self.files.get(name).cloned()
    }

    pub fn files(&self) -> Iter<File<'a, U>> {
        Iter::from(&self.file_list)
    }
    pub fn packages(&self) -> Iter<Package<'a, U>> {
        Iter::from(&self.package_list)
    }
    pub fn node(&self, key: &str) -> Option<Node<'a, U>> {
        self.nodes.get(key).cloned()
    }
}
#[derive(Debug)]
pub struct Ast<'a, U>(Rc<AstDetail<'a, U>>);
impl<'a, U> Ast<'a, U> {
    pub fn util(&self) -> Rc<U> {
        self.0.util.clone()
    }
    pub fn package(&self, name: &str) -> Option<Package<'a, U>> {
        self.0.package(name)
    }
    pub fn file(&self, name: &str) -> Option<File<'a, U>> {
        self.0.file(name)
    }

    pub fn files(&self) -> Iter<File<'a, U>> {
        self.0.files()
    }
    pub fn packages(&self) -> Iter<Package<'a, U>> {
        self.0.packages()
    }
    pub fn node(&self, key: &str) -> Option<Node<'a, U>> {
        self.0.node(key)
    }
    pub fn all_nodes(&self) -> AllNodes<'a, U> {
        AllNodes::from(self)
    }
}
impl<'a, U> Clone for Ast<'a, U> {
    fn clone(&self) -> Self {
        Ast(self.0.clone())
    }
}
impl<'a, U: Util + 'a> Ast<'a, U> {
    pub fn new(source: &'a impl Source<'a>, util: Rc<U>) -> Result<Self, anyhow::Error> {
        let targets: HashSet<String> = source.targets().iter().cloned().collect();
        let mut ast = AstDetail {
            util: util.clone(),
            packages: HashMap::default(),
            files: HashMap::default(),
            file_list: Rc::new(RefCell::new(Vec::new())),
            targets: HashMap::with_capacity(source.targets().len()),
            defined_extensions: Extensions::new(),
            nodes: HashMap::default(),
            package_list: Rc::new(RefCell::new(Vec::new())),
        };

        for fd in source.files() {
            let fd: FileDescriptor<'a> = fd.into();
            let pkg = {
                let name = fd.package();
                ast.packages
                    .entry(name.to_string())
                    .or_insert_with(|| {
                        let pkg = Package::new(name, util.clone());
                        ast.package_list.borrow_mut().push(pkg.clone());
                        pkg
                    })
                    .clone()
            };
            let build_target = targets.contains(fd.name());
            let file = File::new(build_target, fd.to_owned(), pkg.clone())?;
            for d in fd.dependencies() {
                let dep = ast
                    .file(d)
                    .ok_or_else(|| anyhow!("dependency {} has not been hydrated", d))?;

                file.add_dependency(dep.clone());
                dep.add_dependent(file.clone());

                if build_target {
                    ast.targets.insert(file.name().to_string(), file.clone());
                }
                ast.files.insert(file.name().to_string(), file.clone());
                for ext in file.defined_extensions() {
                    ast.defined_extensions.insert(ext)
                }
                pkg.add_file(file.clone());
            }

            for node in file.nodes() {
                ast.nodes.insert(node.fully_qualified_name(), node);
            }

            for msg in file.all_messages() {
                for field in msg.obj_fields() {
                    match field.value_type() {
                        Type::Enum(path) | Type::Message(path) => {
                            let node = ast
                                .nodes
                                .get(path)
                                .cloned()
                                .ok_or_else(|| anyhow!("enum {} not found", path))?;
                            field.set_value(node)?;
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
        let ast = Ast(Rc::new(ast));
        util.init(ast.clone());
        Ok(ast)
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    #[test]
    fn nodes_issue() {
        let pkg = crate::Package::new("foo", Rc::new(crate::util::Generic {}));

        for n in pkg.nodes() {
            println!("{:?}", n);
        }
    }
}
