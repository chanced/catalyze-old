use crate::iter::Iter;
use crate::AllNodes;
use crate::Extensions;
use crate::FileDescriptor;
use crate::Input;
use crate::Node;
use crate::Type;
use crate::{File, Package};

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use anyhow::anyhow;
use anyhow::bail;

/// Ast encapsulates the entirety of the input CodeGeneratorRequest from protoc,
/// parsed to build the Node graph used by catalyze.
#[derive(Debug, Clone)]
pub(crate) struct AstDetail<'a> {
    files: HashMap<String, File<'a>>,
    file_list: Rc<RefCell<Vec<File<'a>>>>,
    targets: HashMap<String, File<'a>>,
    target_files: Rc<RefCell<Vec<File<'a>>>>,
    packages: HashMap<String, Package<'a>>,
    package_list: Rc<RefCell<Vec<Package<'a>>>>,
    defined_extensions: Extensions<'a>,
    nodes: HashMap<String, Node<'a>>,
}
impl<'a> AstDetail<'a> {
    pub fn package(&self, name: &str) -> Option<Package<'a>> {
        self.packages.get(name).cloned()
    }
    pub fn file(&self, name: &str) -> Option<File<'a>> {
        self.files.get(name).cloned()
    }

    pub fn files(&self) -> Iter<File<'a>> {
        Iter::from(&self.file_list)
    }
    pub fn target_files(&self) -> Iter<File<'a>> {
        Iter::from(&self.file_list)
    }
    pub fn packages(&self) -> Iter<Package<'a>> {
        Iter::from(&self.package_list)
    }
    pub fn node(&self, key: &str) -> Option<Node<'a>> {
        self.nodes.get(key).cloned()
    }
}
#[derive(Debug, Clone)]
pub struct Ast<'a>(Rc<AstDetail<'a>>);
impl<'a> Ast<'a> {
    pub fn package(&self, name: &str) -> Option<Package<'a>> {
        self.0.package(name)
    }
    pub fn file(&self, name: &str) -> Option<File<'a>> {
        self.0.file(name)
    }
    pub fn files(&self) -> Iter<File<'a>> {
        self.0.files()
    }
    pub fn target_files(&self) -> Iter<File<'a>> {
        Iter::from(&self.0.target_files)
    }

    pub fn packages(&self) -> Iter<Package<'a>> {
        self.0.packages()
    }
    pub fn node(&self, key: &str) -> Option<Node<'a>> {
        self.0.node(key)
    }
    pub fn all_nodes(&self) -> AllNodes<'a> {
        AllNodes::from(self)
    }
}

impl<'a> Ast<'a> {
    pub fn new(input: Input<'a>) -> Result<Self, anyhow::Error> {
        let mut ast = AstDetail {
            packages: HashMap::default(),
            files: HashMap::default(),
            file_list: Rc::new(RefCell::new(Vec::new())),
            targets: HashMap::with_capacity(input.targets.len()),
            defined_extensions: Extensions::new(),
            nodes: HashMap::default(),
            package_list: Rc::new(RefCell::new(Vec::new())),
            target_files: Rc::new(RefCell::new(Vec::new())),
        };

        for fd in input.files() {
            let fd: FileDescriptor<'a> = fd.into();

            let pkg = {
                let name = fd.package();
                ast.packages
                    .entry(name.to_string())
                    .or_insert_with(|| {
                        let pkg = Package::new(name);
                        ast.package_list.borrow_mut().push(pkg.clone());
                        pkg
                    })
                    .clone()
            };
            let build_target = input.targets.contains(&fd.name().to_string());
            let file = File::new(build_target, fd.to_owned(), pkg.clone())?;
            for d in fd.dependencies() {
                let dep = ast
                    .file(d)
                    .ok_or_else(|| anyhow!("dependency {} has not been hydrated", d))?;

                file.add_import(dep.clone());
                dep.add_dependent(file.clone());

                if build_target {
                    ast.targets.insert(file.name().to_string(), file.clone());
                    ast.target_files.borrow().push(file.clone());
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
        Ok(ast)
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    #[test]
    fn nodes_issue() {
        let pkg = crate::Package::new("foo");

        for n in pkg.nodes() {
            println!("{:?}", n);
        }
    }
}
