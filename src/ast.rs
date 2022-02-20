use crate::container::Container;
use crate::Extension;
use crate::Message;
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

#[derive(Debug)]
pub struct Ast<U> {
    pub targets: HashMap<String, Rc<File<U>>>,
    pub packages: HashMap<String, Rc<Package<U>>>,
    pub nodes: HashMap<String, Node<U>>,
    pub extensions: Vec<Rc<Extension<U>>>,
    pub util: Rc<RefCell<U>>,
    pub file_descriptors: Vec<FileDescriptorProto>,
    pub target_list: HashSet<String>,
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
    pub fn new(source: impl Source, util: Rc<RefCell<U>>) -> Result<Self, anyhow::Error> {
        let target_list = source.targets().cloned().collect::<HashSet<String>>();
        let mut ast = Self {
            util: util.clone(),
            targets: HashMap::with_capacity(target_list.len()),
            target_list,
            packages: HashMap::default(),
            nodes: HashMap::default(),
            extensions: Vec::default(),
            file_descriptors: source
                .files()
                .cloned()
                .collect::<Vec<FileDescriptorProto>>(),
        };
        let mut seen: HashMap<String, Rc<File<U>>> = HashMap::default();
        for fd in ast.file_descriptors.iter().cloned() {
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
            let file = File::new(build_target, fd.clone(), pkg, util.clone());
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
    // fn hydrate_package(&mut self, fd: Rc<FileDescriptorProto>) -> Option<Rc<Package<U>>> {

    // }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_() {}
}

// // process_code_generator_request
// pub fn process_code_generator_request<U>(request: prost_types::compiler::CodeGeneratorRequest, util: U) -> Ast<U> {
//     // let mut ast = Ast {
//     //     util,
//     //     targets: HashMap::with_capacity(request.proto_file.len()),
//     //     packages: HashMap::default(),
//     //     nodes: HashMap::default(),
//     //     extensions: Vec::default(),
//     // };

//     let target_list = request
//         .file_to_generate
//         .iter()
//         .cloned()
//         .collect::<HashSet<String>>();

//     for file in request.proto_file {
//         let pkg = ast.
//     }

// }

// impl<U> Ast<U> {
//     fn hydrate_pkg(
//         &mut self,
//         fd: &prost_types::FileDescriptorProto,
//         util: U,
//     ) -> Option<Rc<Package<U>>> {
//         let p = fd.package();
//         if p.is_empty() {
//             return None;
//         }
//         if self.packages.get(p).is_some() {
//             return Some(self.packages.get(p).unwrap().clone());
//         }
//         let pkg = Package::new(fd.clone(), util);
//         self.packages.insert(fd.package().to_string(), pkg.clone());
//         Some(pkg)
//     }
// }
