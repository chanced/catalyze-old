use crate::{DecodedInput, Input, Module};
use anyhow::bail;
use prost::Message;
use std::{
    fs::File,
    io::{self, stdin},
    io::{stdout, BufReader, BufWriter, Cursor, Read, Stdin, Stdout, Write},
    path::Path,
};
pub trait Workflow: Sized {
    fn decode_input<I: Read>(input: &mut BufReader<I>) -> Result<DecodedInput, io::Error>;
    fn is_standalone(&self) -> bool;
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
    fn is_standalone(&self) -> bool {
        true
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
    fn is_standalone(&self) -> bool {
        false
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
    file_desc_set: Option<prost_types::FileDescriptorSet>,
    code_gen_req: Option<prost_types::compiler::CodeGeneratorRequest>,
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
            file_desc_set: None,
            code_gen_req: None,
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
            file_desc_set: self.file_desc_set,
            code_gen_req: self.code_gen_req,
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
            file_desc_set: self.file_desc_set,
            code_gen_req: self.code_gen_req,
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
                .ok_or_else(|| anyhow::anyhow!("provided path is not valid UTF-8: {:?}", path))?;
            targets.push(p.to_string());
        }
        let output_path = output_path.as_ref();
        let output_path = output_path
            .to_str()
            .ok_or_else(|| {
                anyhow::anyhow!("provided output path is not valid UTF-8: {:?}", output_path)
            })?
            .to_string();

        Ok(Generator {
            targets,
            input: BufReader::new(input),
            modules: Vec::new(),
            workflow: Box::new(Standalone {}),
            output_path: Some(output_path),
            output: None,
            file_desc_set: None,
            code_gen_req: None,
        })
    }
}

impl<'a, W, I, O> Generator<'a, W, I, O>
where
    W: Workflow,
    I: Read,
    O: Write,
{
    fn parse_input(&'_ mut self) -> Result<Input<'_>, anyhow::Error> {
        match W::decode_input(&mut self.input)? {
            DecodedInput::FileDescriptorSet(fds) => {
                self.file_desc_set = Some(fds);
                let fds = self.file_desc_set.as_ref().unwrap();
                let files = fds.file.as_slice();
                let mut input = Input::new(files, "");
                let output_path = self
                    .output_path
                    .clone()
                    .ok_or(anyhow::anyhow!("no output path provided"))?;
                if output_path.is_empty() {
                    bail!("no output path provided");
                }
                input.parmeters.set_output_path(output_path);
                Ok(input)
            }
            DecodedInput::CodeGeneratorRequest(cgr) => {
                self.code_gen_req = Some(cgr);
                let cgr = self.code_gen_req.as_ref().unwrap();
                let files = cgr.proto_file.as_slice();
                self.targets = cgr.file_to_generate.clone();
                let mut input = Input::new(files, cgr.parameter());
                let op = self
                    .output_path
                    .clone()
                    .ok_or(anyhow::anyhow!("no output path provided"))?;
                if op.is_empty() {
                    bail!("no output path provided");
                }
                input.parmeters.set_output_path(op);
                Ok(input)
            }
        }
    }

    fn parse_compiler_vers(&self) -> Result<Option<semver::Version>, anyhow::Error> {
        let vers = self
            .code_gen_req
            .as_ref()
            .and_then(|cgr| cgr.compiler_version.as_ref());
        if let Some(vers) = vers {
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
        } else {
            Ok(None)
        }
    }

    pub fn generate(&mut self) -> Result<(), anyhow::Error> {
        let input = self.parse_input()?;
        let compiler_vers = self.parse_compiler_vers()?;
        let mut output = self.output.take().unwrap();
        let is_standalone = self.workflow.is_standalone();
        Ok(())
    }

    fn generate_standalone(&mut self) -> Result<(), anyhow::Error> {
        let input = self.parse_input()?;
        let compiler_vers = self.parse_compiler_vers()?;
        let mut output = self.output.take().unwrap();
        let is_standalone = self.workflow.is_standalone();
        Ok(())
    }
    fn generate_plugin(&mut self) -> Result<(), anyhow::Error> {}
}
