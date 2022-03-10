use crate::proto::FileDescriptor;
use crate::util::Util;
use crate::Enum;
use crate::Extensions;
use crate::Source;
use crate::{File, Package};

use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

use anyhow::bail;

// protoc
// --include_imports
// --include_source_info
// --proto_path=[dep dir path]
// --descriptor_set_out=[path].bin
// [path].proto

trait Lookup<'a, U> {
    fn must_seen_enum(&self, name: &str) -> Option<Enum<'a, U>>;
    fn must_seen_message(&self, name: &str) -> Option<Enum<'a, U>>;
    fn must_seen_file(&self, name: &str) -> Option<Enum<'a, U>>;
}

/// Ast encapsulates the entirety of the input CodeGeneratorRequest from protoc,
/// parsed to build the Node graph used by catalyze.
#[derive(Debug)]
pub struct Ast<'a, U> {
    files: HashMap<String, File<'a, U>>,
    targets: HashMap<String, File<'a, U>>,
    packages: HashMap<String, Package<'a, U>>,
    defined_extensions: Extensions<'a, U>,
    util: RefCell<Rc<U>>,
}
impl<'a, U> Ast<'a, U> {
    pub fn package(&self, name: &str) -> Option<Package<'a, U>> {
        self.packages.get(name).cloned()
    }
    pub fn file(&self, name: &str) -> Option<File<'a, U>> {
        self.files.get(name).cloned()
    }

    pub fn files(&self) -> &HashMap<String, File<'a, U>> {
        &self.files
    }
    pub fn packages(&self) -> &HashMap<String, Package<'a, U>> {
        &self.packages
    }
}

impl<'a, U: Util> Ast<'a, U> {
    pub fn new(source: &'a impl Source<'a>, util: Rc<U>) -> Result<Self, anyhow::Error> {
        let targets: HashSet<String> = source.targets().iter().cloned().collect();
        let mut ast = Self {
            util: RefCell::new(util.clone()),
            packages: HashMap::default(),
            files: HashMap::default(),
            targets: HashMap::with_capacity(source.targets().len()),
            defined_extensions: Extensions::new(),
        };

        for fd in source.files() {
            let fd: FileDescriptor<'a> = fd.into();
            let pkg = {
                let name = fd.package();
                ast.packages
                    .entry(name.to_string())
                    .or_insert_with(|| Package::new(name, util.clone()))
                    .clone()
            };
            let build_target = targets.contains(fd.name());
            let file = File::new(build_target, fd.to_owned(), pkg.clone());
            for d in fd.dependencies() {
                let dep = match ast.file(d) {
                    Some(f) => f,
                    None => bail!("dependency {} has not been hydrated", d),
                };

                file.add_dependency(dep.clone());
                dep.add_dependent(file.clone());

                if build_target {
                    ast.targets.insert(file.name().to_string(), file.clone());
                }
                ast.files.insert(file.name().to_string(), file.clone());

                for ext in file.defined_extensions() {
                    ast.defined_extensions.insert(ext)
                }
                pkg.add_file(file.clone())
            }
        }
        let util = util.init(&ast)?;
        let util = Rc::new(util);
        for pkg in ast.packages().values() {
            pkg.replace_util(util.clone())
        }
        Ok(ast)
    }
    pub(crate) fn replace_util(&mut self, util: U) {
        self.util.replace(Rc::new(util));
    }
    pub fn util(&self) -> Rc<U> {
        self.util.borrow().clone()
    }
}
