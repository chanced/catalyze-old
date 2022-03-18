use crate::{Ast, DecodedInput, Input, Module};
use anyhow::{anyhow, bail};
use prost::Message;
use prost_types::compiler::CodeGeneratorRequest;

use std::{
    collections::HashMap,
    fs::File,
    io::{self, stdin},
    io::{stdout, BufReader, BufWriter, Cursor, Read, Stdin, Stdout, Write},
    path::Path,
};

pub trait Workflow: Sized {
    fn decode_input<I: Read>(input: &mut BufReader<I>) -> Result<DecodedInput, io::Error>;
}
/// The `Standalone` workflow reads a `FileDescriptorSet`, typically from the
/// contents of a saved output from `protoc`, and generates based on a list of
/// target (proto) files. The output is saved to disk at the specified output
/// path.
pub struct Standalone {}
impl Workflow for Standalone {
    fn decode_input<I: Read>(input: &mut BufReader<I>) -> Result<DecodedInput, io::Error> {
        let mut buf = Vec::new();
        input.read_to_end(&mut buf)?;
        let buf = Cursor::new(buf);
        let fds = prost_types::FileDescriptorSet::decode(buf)?;
        Ok(DecodedInput::FileDescriptorSet(fds))
    }
}

/// ProtocPlugins reads a `CodeGeneratorRequest` from `stdin` and writes a
/// `CodeGeneratorResponse` to `stdout`.
///
/// The input (`stdin`) and output (`stdout`) can be configured, using any
/// reader and writer respectively.
pub struct ProtocPlugin {}
impl Workflow for ProtocPlugin {
    fn decode_input<I: Read>(input: &mut BufReader<I>) -> Result<DecodedInput, io::Error> {
        let mut buf = Vec::new();
        input.read_to_end(&mut buf)?;
        let buf = Cursor::new(buf);
        let cgr = prost_types::compiler::CodeGeneratorRequest::decode(buf)?;
        Ok(DecodedInput::CodeGeneratorRequest(cgr))
    }
}

pub struct Generator<'a, W = ProtocPlugin, I = Stdin, O = Stdout>
where
    W: Workflow,
    I: Read,
    O: Write,
{
    input: BufReader<I>,
    output: Option<BufWriter<O>>,
    output_path: Option<String>,
    modules: Vec<Box<dyn Module<'a>>>,
    targets: Vec<String>,
    workflow: Box<W>,
    has_rendered: bool,
    parsed_input: Option<Input>,
}

impl<'a> Generator<'a> {
    pub fn new_protoc_plugin() -> Generator<'a, ProtocPlugin, Stdin, Stdout> {
        let input = BufReader::new(stdin());
        let output = Some(BufWriter::new(stdout()));
        Generator {
            input,
            output,
            modules: Vec::new(),
            targets: Vec::new(),
            workflow: Box::new(ProtocPlugin {}),
            output_path: None,
            has_rendered: false,
            parsed_input: None,
        }
    }
}

impl<'a, I, O> Generator<'a, ProtocPlugin, I, O>
where
    I: Read,
    O: Write,
{
    pub fn with_input<R: Read>(self, input: R) -> Generator<'a, ProtocPlugin, R, O> {
        Generator {
            input: BufReader::new(input),
            modules: self.modules,
            targets: self.targets,
            workflow: self.workflow,
            output_path: self.output_path,
            output: self.output,
            has_rendered: self.has_rendered,
            parsed_input: self.parsed_input,
        }
    }
    pub fn with_output<W: Write>(self, output: W) -> Generator<'a, ProtocPlugin, I, W> {
        Generator {
            input: self.input,
            modules: self.modules,
            targets: self.targets,
            workflow: self.workflow,
            output_path: self.output_path,
            output: Some(BufWriter::new(output)),
            has_rendered: self.has_rendered,
            parsed_input: self.parsed_input,
        }
    }
    pub fn output_path(&mut self, path: &str) -> &mut Self {
        self.output_path = Some(path.to_string());
        self
    }
}

impl<'a, W, I> Generator<'a, W, I>
where
    W: Workflow,
    I: Read,
{
    /// Returns a new `Generator` with the `Standalone` workflow.
    pub fn new_stand_alone(
        input: I,
        target_protos: &[impl AsRef<Path>],
        output_path: impl AsRef<Path>,
    ) -> Result<Generator<'a, Standalone, I, File>, anyhow::Error> {
        let mut targets = Vec::with_capacity(target_protos.len());
        for path in target_protos {
            let path = path.as_ref();
            let p = path
                .to_str()
                .ok_or_else(|| anyhow!("provided path is not valid UTF-8: {:?}", path))?;
            targets.push(p.to_string());
        }
        let output_path = output_path.as_ref();
        let output_path = output_path
            .to_str()
            .ok_or_else(|| anyhow!("provided output path is not valid UTF-8: {:?}", output_path))?
            .to_string();

        Ok(Generator {
            targets,
            input: BufReader::new(input),
            modules: Vec::new(),
            workflow: Box::new(Standalone {}),
            output_path: Some(output_path),
            output: None,
            has_rendered: false,
            parsed_input: None,
        })
    }
}

impl<'a, W, I, O> Generator<'a, W, I, O>
where
    W: Workflow,
    I: Read,
    O: Write,
{
    fn parse_input(
        buf: &mut BufReader<I>,
        output_path: Option<String>,
    ) -> Result<Input, anyhow::Error> {
        match W::decode_input(buf)? {
            DecodedInput::FileDescriptorSet(fds) => {
                let mut input = Input::new(fds.file, "");
                let output_path = output_path.ok_or_else(|| anyhow!("output path not provided"))?;
                input.parmeters.set_output_path(output_path);
                Ok(input)
            }
            DecodedInput::CodeGeneratorRequest(cgr) => {
                let mut input = Input::new(cgr.proto_file.clone(), cgr.parameter());
                if let Some(op) = output_path {
                    input.parmeters.set_output_path(op);
                }
                input.protoc_version = Self::parse_compiler_vers(cgr.compiler_version.as_ref())?;
                Ok(input)
            }
        }
    }

    fn parse_compiler_vers(
        vers: Option<&prost_types::compiler::Version>,
    ) -> Result<Option<semver::Version>, anyhow::Error> {
        vers.map_or(Ok(None), |vers| {
            let suffix = vers.suffix();
            let pre = if !suffix.is_empty() {
                semver::Prerelease::new(suffix)?
            } else {
                semver::Prerelease::EMPTY
            };
            let vers = semver::Version {
                major: vers.major().try_into()?,
                minor: vers.minor().try_into()?,
                patch: vers.patch().try_into()?,
                pre,
                build: semver::BuildMetadata::EMPTY,
            };
            Ok(Some(vers))
        })
    }

    pub fn render(&'a mut self) -> Result<(), anyhow::Error> {
        if self.has_rendered {
            bail!("generator has already rendered")
        }
        for m in self.modules.iter_mut() {
            m.init();
        }
        let input = Self::parse_input(&mut self.input, self.output_path.clone())?;

        self.parsed_input = Some(input);
        let ast = Ast::new(self.parsed_input.as_ref().unwrap())?;

        let mut artifacts = vec![];
        for m in self.modules.iter_mut() {
            let mut res = m.execute(ast.target_file_map(), ast.clone());
            artifacts.append(&mut res);
        }

        self.has_rendered = true;
        Ok(())
    }
}
