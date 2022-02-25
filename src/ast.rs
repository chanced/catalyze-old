use crate::Enum;
use crate::Extension;
use crate::Node;
use crate::Source;
use crate::{File, Package};

use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

use anyhow::bail;
use prost_types::FileDescriptorProto;

// protoc
// --include_imports
// --include_source_info
// --proto_path=[dep dir path]
// --descriptor_set_out=[path].bin
// [path].proto

trait Lookup<'a, U> {
    fn must_seen_enum(&self, name: &str) -> Option<Rc<Enum<'a, U>>>;
    fn must_seen_message(&self, name: &str) -> Option<Rc<Enum<'a, U>>>;
    fn must_seen_file(&self, name: &str) -> Option<Rc<Enum<'a, U>>>;
}

#[derive(Debug)]
pub struct Ast<'a, U> {
    pub targets: HashMap<String, File<'a, U>>,
    pub packages: HashMap<String, Package<'a, U>>,
    pub nodes: HashMap<String, Node<'a, U>>,
    pub extensions: Vec<Rc<Extension<'a, U>>>,
    pub util: Rc<RefCell<U>>,
    pub file_descriptors: Vec<&'a FileDescriptorProto>,
    pub target_list: HashSet<String>,
}

/// Ast encapsulates the entirety of the input CodeGeneratorRequest from protoc,
/// parsed to build the Node graph used by catalyze.
impl<'a, U> Ast<'a, U> {
    pub fn package(&self, name: &str) -> Option<Package<'a, U>> {
        self.packages.get(name).cloned()
    }
    pub fn file(&self, name: &str) -> Option<File<'a, U>> {
        self.targets.get(name).cloned()
    }
}

impl<'a, U> Ast<'a, U> {
    pub fn new(source: &'a impl Source<'a>, util: Rc<RefCell<U>>) -> Result<Self, anyhow::Error> {
        let target_list = source
            .targets()
            .iter()
            .cloned()
            .collect::<HashSet<String>>();
        let file_descriptors = source.files().iter().collect();
        let mut ast = Self {
            util: util.clone(),
            targets: HashMap::with_capacity(target_list.len()),
            target_list,
            packages: HashMap::default(),
            nodes: HashMap::default(),
            extensions: Vec::default(),
            file_descriptors,
        };
        let mut seen: HashMap<String, File<'a, U>> = HashMap::default();

        for fd in ast.file_descriptors.iter() {
            let pkg = {
                let name = fd.package();
                if name.is_empty() {
                    None
                } else {
                    Some(
                        ast.packages
                            .entry(name.to_string())
                            .or_insert_with(|| Package::new(name, util.clone()))
                            .clone(),
                    )
                }
            };
            let build_target = ast.target_list.contains(fd.name());
            let file = File::new(build_target, fd, pkg, util.clone());
            ast.targets.insert(fd.name().to_string(), file.clone());
            for d in fd.dependency.iter() {
                let dep = match seen.get(d).cloned() {
                    Some(f) => f,
                    None => bail!("dependency {} has not been hydrated", d),
                };
                file.add_dependency(dep.clone());
                dep.add_dependent(file.clone());
            }
            seen.insert(fd.name().to_string(), file.clone());
        }

        Ok(ast)
    }
    // fn hydrate_package(&mut self, fd: Rc<FileDescriptorProto>) -> Option<Package<'a, U>> {

    // }
}
