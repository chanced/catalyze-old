use crate::container::Container;
use crate::iter::Iter;
use crate::AllNodes;
use crate::Extensions;
use crate::Field;
use crate::FileDescriptor;
use crate::Input;
use crate::Method;
use crate::MethodIO;
use crate::Node;
use crate::Type;
use crate::{File, Package};

use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;
use std::slice;

use anyhow::anyhow;
use anyhow::bail;
/// Ast encapsulates the entirety of the input CodeGeneratorRequest from protoc,
/// parsed to build the Node graph used by catalyze.
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
        self.0.target_files()
    }
    pub(crate) fn target_file_map(&self) -> HashMap<String, File<'a>> {
        let targets = self.target_files();

        let mut map = HashMap::with_capacity(targets.len());
        for target in targets {
            map.insert(target.name().to_string(), target);
        }
        map
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
    pub fn new(input: &'a Input) -> Result<Self, anyhow::Error> {
        let ast = AstDetail {
            packages: HashMap::default(),
            files: HashMap::default(),
            file_list: Rc::new(RefCell::new(Vec::new())),
            targets: HashMap::with_capacity(input.targets().len()),
            defined_extensions: Extensions::new(),
            nodes: HashMap::default(),
            package_list: Rc::new(RefCell::new(Vec::new())),
            target_files: Rc::new(RefCell::new(Vec::new())),
            target_list: input.targets().iter().cloned().collect(),
        }
        .hydrate_files(input.files().iter())?;
        let ast = Ast(Rc::new(ast));
        Ok(ast)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct AstDetail<'a> {
    files: HashMap<String, File<'a>>,
    file_list: Rc<RefCell<Vec<File<'a>>>>,
    target_list: HashSet<String>,
    targets: HashMap<String, File<'a>>,
    target_files: Rc<RefCell<Vec<File<'a>>>>, // todo: remove
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
        Iter::from(&self.target_files)
    }
    pub fn packages(&self) -> Iter<Package<'a>> {
        Iter::from(&self.package_list)
    }
    pub fn node(&self, key: &str) -> Option<Node<'a>> {
        self.nodes.get(key).cloned()
    }
    fn connect_file_dep(&mut self, file: File<'a>, dep: &str) -> anyhow::Result<()> {
        let dep = self
            .file(dep)
            .ok_or_else(|| anyhow!("dependency {} has not been hydrated", dep))?;

        file.add_import(dep.clone());
        dep.add_dependent(file.clone());
        Ok(())
    }
    fn hydrate_extensions(&self, container: Container<'a>) -> anyhow::Result<()> {
        for ext in container.defined_extensions() {
            let extendee = self
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
        Ok(())
    }

    fn connect_file_deps(
        &mut self,
        file: File<'a>,
        deps: slice::Iter<String>,
    ) -> anyhow::Result<()> {
        for d in deps {
            self.connect_file_dep(file.clone(), d)?;
        }
        Ok(())
    }
    fn load_pkg(&mut self, fd: FileDescriptor<'a>) -> Package<'a> {
        self.packages
            .entry(fd.name().to_string())
            .or_insert_with(|| {
                let pkg = Package::new(fd.package());
                self.package_list.borrow_mut().push(pkg.clone());
                pkg
            })
            .clone()
    }
    fn is_build_target(&self, fd: FileDescriptor<'a>) -> bool {
        self.target_list.contains(&fd.name().to_string())
    }
    fn hydrate_method(&self, meth: Method<'a>) -> anyhow::Result<()> {
        let hydrate = |p: MethodIO| {
            if p.is_empty() {
                bail!("method {} missing {}", meth.fully_qualified_name(), p);
            }
            let node = self.node(p.node_name()).ok_or_else(|| {
                anyhow!(
                    "method {} {} type {} is not hydrated",
                    meth.fully_qualified_name(),
                    p,
                    p.node_name()
                )
            })?;
            if let Node::Message(msg) = node {
                match p {
                    MethodIO::Input(_) => {
                        meth.set_input(msg);
                    }
                    MethodIO::Output(_) => {
                        meth.set_output(msg);
                    }
                }
                Ok(())
            } else {
                bail!(
                    "method {} has an invalid {} type {}",
                    meth.fully_qualified_name(),
                    p,
                    node
                )
            }
        };

        let (i, o) = meth.io();

        hydrate(i)?;
        hydrate(o)?;
        Ok(())
    }

    fn get_node(&self, fqn: &str) -> anyhow::Result<Node<'a>> {
        self.node(fqn)
            .ok_or_else(|| anyhow!("node {} not found", fqn))
    }

    fn hydrate_enum_field(&self, field: Field<'a>, node: Node<'a>) -> anyhow::Result<()> {
        let enm = match node.clone() {
            Node::Enum(enm) => enm,
            _ => bail!("node {} is not an enum", node.fully_qualified_name()),
        };
        let msg = field.message();
        field.set_value(node.clone())?;
        node.add_dependent(msg.clone());
        msg.register_import(enm.file());
        Ok(())
    }
    fn hydrate_embed_field(
        &self,
        idx: usize,
        field: Field<'a>,
        node: Node<'a>,
    ) -> anyhow::Result<()> {
        let embed = node.clone().as_message()?;
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

    fn hydrate_field(&self, idx: usize, field: Field<'a>) -> anyhow::Result<()> {
        match field.value_type() {
            Type::Enum(path) => self.hydrate_enum_field(field.clone(), self.get_node(path)?),
            Type::Message(path) => {
                self.hydrate_embed_field(idx, field.clone(), self.get_node(path)?)
            }
            _ => Ok(()),
        }
    }
    fn add_file(&mut self, file: File<'a>) {
        self.nodes
            .insert(file.name().to_string(), file.clone().into());
        self.files.insert(file.name().to_string(), file.clone());
        self.file_list.borrow_mut().push(file.clone());
        if file.build_target() {
            self.targets.insert(file.name().to_string(), file.clone());
            self.target_files.borrow_mut().push(file.clone());
        }
    }
    fn hydrate_files(
        mut self,
        files: slice::Iter<'a, prost_types::FileDescriptorProto>,
    ) -> anyhow::Result<Self> {
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

    fn init_file(&mut self, fd: FileDescriptor<'a>) -> anyhow::Result<File<'a>> {
        let pkg = self.load_pkg(fd);
        let build_target = self.is_build_target(fd);
        let file = File::new(build_target, fd.to_owned(), pkg.clone())?;
        self.connect_file_deps(file.clone(), fd.dependencies())?;
        for ext in file.defined_extensions() {
            self.defined_extensions.insert(ext)
        }
        pkg.add_file(file.clone());
        for node in file.all_nodes() {
            self.nodes.insert(node.fully_qualified_name(), node.clone());
            if let Node::Message(msg) = node {
                for (fqn, map) in msg.maps() {
                    self.nodes.insert(fqn, map.into());
                }
            }
        }
        Ok(file)
    }
}
