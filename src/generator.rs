use anyhow::bail;

use crate::Module;
use std::{
    fs::File,
    io::{self, stdin},
    io::{stdout, BufReader, BufWriter, Read, Stdin, Stdout, Write},
    marker::PhantomData,
    path::{Path, PathBuf},
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

/// The `StandAlone` workflow reads a `FileDescriptorSet`, typically from the
/// contents of a saved output from `protoc`, and generates based on a list of
/// target (proto) files. The output is saved to disk at the specified output
/// path.
pub struct StandAlone {}
impl Workflow for StandAlone {
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

pub struct Generator<W: Workflow = ProtocPlugin, I: Read = Stdin, O: Write = Stdout> {
    input: BufReader<I>,
    output: Option<BufWriter<O>>,
    output_dir: Option<PathBuf>,
    modules: Vec<Module>,
    targets: Vec<String>,
    _marker: std::marker::PhantomData<W>,
}

impl Generator {
    pub fn new_protoc_plugin() -> Generator<ProtocPlugin, Stdin, Stdout> {
        let input = BufReader::new(stdin());
        let output = Some(BufWriter::new(stdout()));
        Generator {
            input,
            output,
            modules: Vec::new(),
            targets: Vec::new(),
            _marker: std::marker::PhantomData,
            output_dir: None,
        }
    }
}

impl<I: Read, O: Write> Generator<ProtocPlugin, I, O> {
    pub fn input<R: Read>(self, input: R) -> Generator<ProtocPlugin, R, O> {
        Generator {
            input: BufReader::new(input),
            modules: self.modules,
            targets: self.targets,
            _marker: std::marker::PhantomData,
            output_dir: self.output_dir,
            output: self.output,
        }
    }
    pub fn output<W: Write>(self, output: W) -> Generator<ProtocPlugin, I, W> {
        Generator {
            input: self.input,
            modules: self.modules,
            targets: self.targets,
            _marker: std::marker::PhantomData,
            output_dir: self.output_dir,
            output: Some(BufWriter::new(output)),
        }
    }
}

impl<W: Workflow, I: Read> Generator<W, I> {
    /// Returns a new `Generator` with the `StandAlone` workflow.
    pub fn new_stand_alone(
        input: I,
        target_protos: &[impl AsRef<Path>],
        output_dir: impl AsRef<Path>,
    ) -> Result<Generator<StandAlone, I, File>, anyhow::Error> {
        let mut targets = Vec::with_capacity(target_protos.len());
        for path in target_protos {
            let path = path.as_ref();
            let p = path.to_str();
            if p.is_none() {
                bail!("provided path is not valid UTF-8: {:?}", path);
            }
            targets.push(p.unwrap().to_string());
        }
        Ok(Generator {
            targets,
            input: BufReader::new(input),
            modules: Vec::new(),
            _marker: PhantomData,
            output_dir: Some(output_dir.as_ref().to_path_buf()),
            output: None,
        })
    }
}
