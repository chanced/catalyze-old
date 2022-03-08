use crate::proto::FileDescriptor;
use crate::util::Util;
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
#[derive(Debug, Clone)]
pub struct Ast<'a, U> {
    pub target_files: HashMap<String, File<'a, U>>,
    pub packages: HashMap<String, Package<'a, U>>,
    pub nodes: HashMap<String, Node<'a, U>>,
    pub extensions: Vec<Extension<'a, U>>,
    util: RefCell<Rc<U>>,
    targets: HashSet<String>,
    fds: HashMap<String, FileDescriptor<'a>>,
    ord_fds: Vec<FileDescriptor<'a>>,
}

impl<'a, U> Ast<'a, U> {
    pub fn package(&self, name: &str) -> Option<Package<'a, U>> {
        self.packages.get(name).cloned()
    }
    pub fn file(&self, name: &str) -> Option<File<'a, U>> {
        self.target_files.get(name).cloned()
    }
}

impl<'a, U: Util> Ast<'a, U> {
    pub fn new(source: &'a impl Source<'a>, util: Rc<U>) -> Result<Self, anyhow::Error> {
        let fl = source.files();
        let mut fds: HashMap<String, FileDescriptor<'a>> = HashMap::with_capacity(fl.len());
        let mut ord_fds: Vec<FileDescriptor<'a>> = Vec::with_capacity(fl.len());
        for f in fl {
            let name = f.name();
            let fd = fds.entry(name.to_string()).or_insert_with(|| f.into());
            ord_fds.push(*fd);
        }
        let mut ast = Self {
            util: RefCell::new(util.clone()),
            fds,
            ord_fds,
            targets: source.targets().iter().cloned().collect(),
            packages: HashMap::default(),
            nodes: HashMap::default(),
            extensions: Vec::default(),
            target_files: HashMap::with_capacity(source.targets().len()),
        };

        let mut seen: HashMap<String, File<'a, U>> = HashMap::default();
        for (fd_name, fd) in ast.fds.iter() {
            let pkg = {
                let name = fd.package();
                ast.packages
                    .entry(name.to_string())
                    .or_insert_with(|| Package::new(fd_name, util.clone()))
                    .clone()
            };

            let build_target = ast.targets.contains(fd.name());
            let file = File::new(build_target, fd.to_owned(), pkg);
            ast.target_files.insert(fd.name().to_string(), file.clone());

            for d in fd.dependencies() {
                let dep = match seen.get(d).cloned() {
                    Some(f) => f,
                    None => bail!("dependency {} has not been hydrated", d),
                };
                file.add_dependency(dep.clone());
                dep.add_dependent(file.clone());
            }
            seen.insert(fd.name().to_string(), file.clone());
        }
        let ur = util.init(&ast);
        match ur {
            Ok(util) => {
                ast.replace_util(util);
                Ok(ast)
            }
            Err(e) => {
                let error = Box::new(e);
                Err(anyhow::Error::new(error).context("failed to initialize util"))
            }
        }
    }
    pub(crate) fn replace_util(&mut self, util: U) {
        self.util.replace(Rc::new(util));
    }
    pub fn util(&self) -> Rc<U> {
        self.util.borrow().clone()
    }
}
