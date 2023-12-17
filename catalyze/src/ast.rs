use protobuf::descriptor::field_descriptor_proto::Type;
use protobuf::reflect::FileDescriptor;

use crate::container::Container;
use crate::AllNodes;
use crate::Error;
use crate::Extensions;
use crate::Field;
use crate::Iter;
use crate::Kind;
use crate::Method;
use crate::MethodIo;
use crate::Node;
use crate::{File, Package};

use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::PathBuf;
use std::rc::Rc;
use std::slice;

/// Ast encapsulates the entirety of the input CodeGeneratorRequest from protoc,
/// parsed to build the Node graph used by catalyze.
#[derive(Debug, Clone)]
pub struct Ast(Rc<AstDetail>);
impl Ast {
    #[inline]
    pub fn package(&self, name: &str) -> Option<Package> {
        self.0.package(name)
    }
    #[inline]
    pub fn file(&self, name: &str) -> Option<File> {
        self.0.file(name)
    }
    #[inline]
    pub fn files(&self) -> Iter<File> {
        self.0.files()
    }
    #[inline]
    pub fn target_files(&self) -> Iter<File> {
        self.0.target_files()
    }
    pub(crate) fn target_file_map(&self) -> HashMap<String, File> {
        let targets = self.target_files();
        let mut map = HashMap::with_capacity(targets.len());
        for target in targets {
            map.insert(target.name().to_string(), target);
        }
        map
    }

    #[inline]
    pub fn packages(&self) -> Iter<Package> {
        self.0.packages()
    }

    #[inline]
    pub fn node(&self, key: &str) -> Option<Node> {
        self.0.node(key)
    }

    #[inline]
    pub fn all_nodes(&self) -> AllNodes {
        AllNodes::from(self)
    }
}

impl Ast {
    // pub fn new(input: &'a Input) -> Result<Self, Error> {
    //     let ast = AstDetail {
    //         packages: HashMap::default(),
    //         files: HashMap::default(),
    //         file_list: Rc::new(RefCell::new(Vec::new())),
    //         targets: HashMap::with_capacity(input.targets().len()),
    //         defined_extensions: Extensions::new(),
    //         nodes: HashMap::default(),
    //         package_list: Rc::new(RefCell::new(Vec::new())),
    //         target_files: Rc::new(RefCell::new(Vec::new())),
    //         target_list: input.targets().iter().map(|v| v.as_str()).collect(),
    //     }
    //     .hydrate_files(input.files().iter())?;
    //     let ast = Ast(Rc::new(ast));
    //     Ok(ast)
    // }
}

#[derive(Debug, Clone)]
pub(crate) struct AstDetail {
    files: HashMap<PathBuf, File>,
    file_list: Rc<RefCell<Vec<File>>>,
    target_list: HashSet<String>,
    targets: HashMap<String, File>,
    target_files: Rc<RefCell<Vec<File>>>, // todo: remove
    packages: HashMap<String, Package>,
    package_list: Rc<RefCell<Vec<Package>>>,
    defined_extensions: Extensions,
    nodes: HashMap<String, Node>,
}
impl AstDetail {
    pub fn package(&self, name: &str) -> Option<Package> {
        self.packages.get(name).cloned()
    }
    pub fn file(&self, name: &str) -> Option<File> {
        self.files.get(name).cloned()
    }

    pub fn files(&self) -> Iter<File> {
        Iter::from(&self.file_list)
    }
    pub fn target_files(&self) -> Iter<File> {
        Iter::from(&self.target_files)
    }
    pub fn packages(&self) -> Iter<Package> {
        Iter::from(&self.package_list)
    }
    pub fn node(&self, key: &str) -> Option<Node> {
        self.nodes.get(key).cloned()
    }

    fn connect_file_dep(&mut self, file: File, dep: &str) -> Result<(), Error> {
        let dep = self.file(dep).ok_or_else(|| Error::DependencyNotFound {
            dependency: dep.to_string(),
            dependee: file,
        })?;

        file.add_import(dep.clone());
        dep.add_dependent(file.clone());
        Ok(())
    }

    fn hydrate_extensions(&self, container: Container) -> Result<(), Error> {
        for ext in container.defined_extensions() {
            let extendee =
                self.node(ext.descriptor().extendee())
                    .ok_or_else(|| Error::ExtendeeNotFound {
                        extendee: ext.descriptor().extendee().to_string(),
                    })?;

            let Node::Message(m) = extendee else {
                return Err(Error::InvalidNode {
                    expected: Kind::Message,
                    node: extendee,
                });
            };
            m.add_applied_extension(ext);
        }
        Ok(())
    }

    fn connect_file_deps(&mut self, file: File, deps: slice::Iter<String>) -> Result<(), Error> {
        for d in deps {
            self.connect_file_dep(file.clone(), d)?;
        }
        Ok(())
    }
    fn load_pkg(&mut self, fd: FileDescriptor) -> Package {
        self.packages
            .entry(fd.name().to_string())
            .or_insert_with(|| {
                let pkg = Package::new(fd.package());
                self.package_list.borrow_mut().push(pkg.clone());
                pkg
            })
            .clone()
    }
    fn is_build_target(&self, fd: FileDescriptor) -> bool {
        self.target_list.contains(&fd.name())
    }

    fn hydrate_method(&self, method: Method) -> Result<(), Error> {
        let hydrate = |io: MethodIo| {
            if io.is_empty() {
                return Err(Error::MissingMethod {
                    fully_qualified_name: method.fully_qualified_name().to_string(),
                    method_io: io,
                });
            }

            let node = self
                .node(io.node_name())
                .ok_or_else(|| Error::MissingMethod {
                    fully_qualified_name: method.fully_qualified_name().to_string(),
                    method_io: io,
                })?;

            if let Node::Message(msg) = node {
                match io {
                    MethodIo::Input(_) => {
                        method.set_input(msg);
                    }
                    MethodIo::Output(_) => {
                        method.set_output(msg);
                    }
                }
                Ok(())
            } else {
                return Err(Error::InvalidNode {
                    expected: Kind::Message,
                    node,
                });
            }
        };

        let (i, o) = method.io();

        hydrate(i)?;
        hydrate(o)?;
        Ok(())
    }

    fn get_node(&self, fqn: &str) -> Result<Node, Error> {
        self.node(fqn).ok_or_else(|| Error::NodeNotFound {
            fully_qualified_name: fqn.to_string(),
        })
    }

    fn hydrate_enum_field(&self, field: Field, node: Node) -> Result<(), Error> {
        let enm = match node.clone() {
            Node::Enum(enm) => enm,
            _ => {
                return Err(Error::InvalidNode {
                    expected: Kind::Enum,
                    node,
                })
            }
        };
        let msg = field.message();
        field.set_value(node.clone())?;
        node.add_dependent(msg.clone());
        msg.register_import(enm.file());
        Ok(())
    }
    fn hydrate_embed_field(&self, idx: usize, field: Field, node: Node) -> Result<(), Error> {
        let embed = node.try_into_message().map_err(|node| Error::InvalidNode {
            expected: Kind::Message,
            node,
        })?;
        let msg = field.message();
        field.set_value(node.clone())?;

        if embed.is_map_entry() {
            self.hydrate_field(1, embed.fields().get(1).unwrap())?;
            msg.replace_field(idx, field.into_map()?);
        }
        node.add_dependent(msg.clone());
        msg.register_import(embed.file());
        Ok(())
    }

    fn hydrate_field(&self, idx: usize, field: Field) -> Result<(), Error> {
        match field.value_type() {
            Type::Enum(path) => self.hydrate_enum_field(field.clone(), self.get_node(&path)?),
            Type::Message(path) => {
                self.hydrate_embed_field(idx, field.clone(), self.get_node(&path)?)
            }
            _ => Ok(()),
        }
    }
    fn add_file(&mut self, file: File) {
        self.nodes
            .insert(file.name().to_string(), file.clone().into());
        self.files.insert(file.name(), file.clone());
        self.file_list.borrow_mut().push(file.clone());
        if file.build_target() {
            self.targets.insert(file.name().to_string(), file.clone());
            self.target_files.borrow_mut().push(file.clone());
        }
    }
    fn hydrate_files(
        mut self,
        files: slice::Iter<'_, protobuf::descriptor::FileDescriptorProto>,
    ) -> Result<Self, Error> {
        for fd in files {
            let file = self.init_file(fd.into())?;
            self.add_file(file);
        }
        for file in self.files() {
            self.hydrate_extensions(file.clone().into())?;
            for msg in file.all_messages() {
                self.hydrate_extensions(msg.clone().into())?;
                for (idx, field) in msg.fields().enumerate() {
                    self.hydrate_field(idx, field)?;
                }
            }
            for svc in file.services() {
                for meth in svc.methods() {
                    self.hydrate_method(meth)?;
                }
            }
        }

        Ok(self)
    }

    fn init_file(&mut self, fd: FileDescriptor) -> Result<File, Error> {
        let pkg = self.load_pkg(fd);
        let build_target = self.is_build_target(fd);
        let file = File::new(build_target, fd.to_owned(), pkg.clone())?;
        self.connect_file_deps(file.clone(), fd.dependencies())?;
        for ext in file.defined_extensions() {
            self.defined_extensions.insert(ext)
        }
        pkg.add_file(file.clone());
        for node in file.all_nodes() {
            self.nodes
                .insert(node.fully_qualified_name().to_string(), node.clone());
            if let Node::Message(msg) = node {
                for (fqn, map) in msg.maps() {
                    self.nodes.insert(fqn.to_string(), map.into());
                }
            }
        }
        Ok(file)
    }
}
