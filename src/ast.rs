use crate::iter::Iter;
use crate::util::Util;
use crate::AllNodes;
use crate::Extensions;
use crate::FileDescriptor;
use crate::Node;
use crate::Source;
use crate::Type;
use crate::{File, Package};

use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

use anyhow::anyhow;
use anyhow::bail;

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

                file.add_import(dep.clone());
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
            ast.nodes
                .insert(file.name().to_string(), file.clone().into());

            for node in file.all_nodes() {
                ast.nodes.insert(node.fully_qualified_name(), node.clone());
                if let Node::Method(mth) = node.clone() {
                    if !mth.output_type().is_empty() {
                        let output = ast.node(mth.output_type()).ok_or_else(|| {
                            anyhow!(
                                "method {} has an invalid output type {}",
                                mth.fully_qualified_name(),
                                mth.output_type()
                            )
                        })?;

                        if let Node::Message(output) = output {
                            mth.set_output(output);
                        } else {
                            bail!(
                                "method {} has an invalid output type {}",
                                mth.fully_qualified_name(),
                                mth.output_type()
                            )
                        }
                    }
                    if !mth.input_type().is_empty() {
                        let input = ast.node(mth.output_type()).ok_or_else(|| {
                            anyhow!(
                                "method {} has an invalid output type {}",
                                mth.fully_qualified_name(),
                                mth.output_type()
                            )
                        })?;
                        if let Node::Message(input) = input {
                            mth.set_input(input);
                        } else {
                            bail!(
                                "method {} has an invalid input type {}",
                                mth.fully_qualified_name(),
                                mth.output_type()
                            )
                        }
                    }
                }

                if let Node::Field(field) = node.clone() {
                    match field.value_type() {
                        Type::Enum(path) | Type::Message(path) => {
                            let msg = field.message();
                            let node = ast
                                .nodes
                                .get(path)
                                .cloned()
                                .ok_or_else(|| anyhow!("Node {} not found", path))?;
                            node.add_dependent(msg.clone());
                            field.set_value(node.clone())?;
                            let node_file = match node {
                                Node::Message(m) => m.file(),
                                Node::Enum(e) => e.file(),
                                _ => bail!("Node {} is not a message or enum", path),
                            };
                            msg.register_import(node_file)
                        }
                        _ => continue,
                    }
                }
            }
        }
        for file in ast.files() {
            for ext in file.defined_extensions() {
                let extendee = ast
                    .node(ext.descriptor().extendee())
                    .ok_or_else(|| anyhow!("extendee {} not found", ext.descriptor().extendee()))?;

                if let Node::Message(m) = extendee {
                    m.add_applied_extension(ext.clone());
                } else {
                    bail!(
                        "unexpected extendee type. Expected Message, received {}",
                        extendee
                    )
                }
            }
            for msg in file.all_messages() {
                for ext in msg.defined_extensions() {
                    let extendee = ast.node(ext.descriptor().extendee()).ok_or_else(|| {
                        anyhow!("extendee {} not found", ext.descriptor().extendee())
                    })?;

                    if let Node::Message(m) = extendee {
                        m.add_applied_extension(ext.clone());
                    } else {
                        bail!(
                            "unexpected extendee type. Expected Message, received {}",
                            extendee
                        )
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
