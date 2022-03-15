use anyhow::bail;

use crate::Module;
use std::{
    fs::File,
    io::{self, stdin},
    io::{BufReader, Read, Stdin},
    path::Path,
};

pub struct Input<'a> {
    pub files: &'a [prost_types::FileDescriptorProto],
    pub targets: Vec<String>,
}
impl<'a> Input<'a> {
    pub fn files(&self) -> std::slice::Iter<'a, prost_types::FileDescriptorProto> {
        self.files.iter()
    }
}
pub trait Workflow: Sized {
    fn parse_input<'a, R: Read>(g: Generator<Self, R>) -> Result<Input<'a>, io::Error>;
}

/// A `Standalone` reads a `FileDescriptorSet`, typically from the contents of a
/// saved output from `protoc`, and generates based on a list of target (proto)
/// files. The output is saved to disk at the specified output path.
pub struct Standalone {}
impl Workflow for Standalone {
    fn parse_input<'a, R: Read>(g: Generator<Self, R>) -> Result<Input<'a>, io::Error> {
        todo!()
    }
}

/// ProtocPlugins reads a `CodeGeneratorRequest` from `stdin` and writes a
/// `CodeGeneratorResponse` to `stdout`.
///
/// The input (`stdin`) and output (`stdout`) can be configured, using any
/// reader and writer respectively.
pub struct ProtocPlugin {}
impl Workflow for ProtocPlugin {
    fn parse_input<'a, R: Read>(g: Generator<Self, R>) -> Result<Input<'a>, io::Error> {
        todo!()
    }
}

pub struct Generator<W: Workflow = ProtocPlugin, R: Read = Stdin> {
    input: BufReader<R>,
    modules: Vec<Module>,
    tarets: Vec<String>,
    workflow: Box<W>,
}

impl Generator {
    pub fn new_protoc_plugin() -> Generator<ProtocPlugin> {
        let input = BufReader::new(stdin());
        Self {
            input,
            modules: Vec::new(),
            tarets: Vec::new(),
            workflow: Box::new(ProtocPlugin {}),
        }
    }
}

impl<T: Workflow, R: Read> Generator<T, R> {
    /// Creates a new StandAlone
    pub fn new_standalone(
        input: R,
        targets: &[impl AsRef<Path>],
    ) -> Result<Generator<Standalone, R>, anyhow::Error> {
        let mut target_strs = Vec::with_capacity(targets.len());
        for path in targets {
            let path = path.as_ref();
            let p = path.to_str();
            if p.is_none() {
                bail!("provided path is not valid UTF-8: {:?}", path);
            }
            target_strs.push(p.unwrap().to_string());
        }

        Ok(Generator {
            input: BufReader::new(input),
            modules: Vec::new(),
            tarets: target_strs,
            workflow: Box::new(Standalone {}),
        })
    }
}
