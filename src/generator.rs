use crate::{Ast, Input, Module, ParamMutatorFn, Parameters, Source};
use anyhow::{anyhow, bail};
use prost::Message;

use std::{
    cell::RefCell,
    fs,
    io::{self, stdin},
    io::{stdout, BufReader, BufWriter, Cursor, Read, Stdin, Stdout, Write},
    path::Path,
    rc::Rc,
};

pub trait Workflow: Sized {
    fn decode_source<I: Read>(input: &mut BufReader<I>) -> Result<Source, io::Error>;
}
/// The `Standalone` workflow reads a `FileDescriptorSet`, typically from the
/// contents of a saved output from `protoc`, and generates based on a list of
/// target (proto) files. The output is saved to disk at the specified output
/// path.

pub struct Standalone {}
impl Workflow for Standalone {
    fn decode_source<I: Read>(input: &mut BufReader<I>) -> Result<Source, io::Error> {
        let mut buf = Vec::new();
        input.read_to_end(&mut buf)?;
        let buf = Cursor::new(buf);
        let fds = prost_types::FileDescriptorSet::decode(buf)?;
        Ok(Source::FileDescriptorSet(fds))
    }
}

/// ProtocPlugins reads a `CodeGeneratorRequest` from `stdin` and writes a
/// `CodeGeneratorResponse` to `stdout`.
///
/// The input (`stdin`) and output (`stdout`) can be configured, using any
/// reader and writer respectively.
pub struct ProtocPlugin {}
impl Workflow for ProtocPlugin {
    fn decode_source<I: Read>(input: &mut BufReader<I>) -> Result<Source, io::Error> {
        let mut buf = Vec::new();
        input.read_to_end(&mut buf)?;
        let buf = Cursor::new(buf);
        let cgr = prost_types::compiler::CodeGeneratorRequest::decode(buf)?;
        Ok(Source::CodeGeneratorRequest(cgr))
    }
}

pub struct Generator<W = ProtocPlugin, I = Stdin, O = Stdout>
where
    W: Workflow,
    I: Read,
    O: Write,
{
    input: BufReader<I>,
    output: Option<BufWriter<O>>,
    output_path: Option<String>,
    modules: Vec<Box<dyn Module + 'static>>,
    targets: Vec<String>,
    workflow: Box<W>,
    has_rendered: bool,
    parsed_input: Input,
    param_mutators: Vec<ParamMutatorFn>,
}

impl<'a> Generator {
    pub fn new_protoc_plugin() -> Generator<ProtocPlugin, Stdin, Stdout> {
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
            parsed_input: Input::default(),
            param_mutators: Vec::new(),
        }
    }
}

impl<'a, I, O> Generator<ProtocPlugin, I, O>
where
    I: Read,
    O: Write,
{
    pub fn input<R: Read>(self, input: R) -> Generator<ProtocPlugin, R, O> {
        Generator {
            input: BufReader::new(input),
            modules: self.modules,
            targets: self.targets,
            workflow: self.workflow,
            output_path: self.output_path,
            output: self.output,
            has_rendered: self.has_rendered,
            parsed_input: self.parsed_input,
            param_mutators: self.param_mutators,
        }
    }
    pub fn output<W: Write>(self, output: W) -> Generator<ProtocPlugin, I, W> {
        Generator {
            input: self.input,
            modules: self.modules,
            targets: self.targets,
            workflow: self.workflow,
            output_path: self.output_path,
            output: Some(BufWriter::new(output)),
            has_rendered: self.has_rendered,
            parsed_input: self.parsed_input,
            param_mutators: self.param_mutators,
        }
    }
    pub fn output_path(&mut self, path: &str) -> &mut Self {
        self.output_path = Some(path.to_string());
        self
    }
}

impl<'a, I> Generator<Standalone, I>
where
    I: Read,
{
    /// Returns a new `Generator` with the `Standalone` workflow.
    pub fn new_standalone(
        input: I,
        target_protos: &[impl AsRef<Path>],
        output_path: impl AsRef<Path>,
    ) -> Result<Generator<Standalone, I, fs::File>, anyhow::Error> {
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
            parsed_input: Default::default(),
            param_mutators: Vec::new(),
        })
    }
}

impl<'a, W, I, O> Generator<W, I, O>
where
    W: Workflow,
    I: Read,
    O: Write,
{
    pub fn render(&'a mut self) -> Result<(), anyhow::Error> {
        if self.has_rendered {
            bail!("generator has already rendered")
        }
        let input = Self::parse_input(
            &mut self.input,
            self.output_path.clone(),
            self.targets.clone(),
            self.param_mutators.as_slice(),
        )?;
        self.parsed_input = input;
        for m in self.modules.iter_mut() {
            m.init();
        }

        let ast = Ast::new(&self.parsed_input)?;

        let mut artifacts = vec![];
        for m in self.modules.iter_mut() {
            let mut res = m.execute(ast.target_file_map(), ast.clone());
            artifacts.append(&mut res);
        }
        self.has_rendered = true;
        Ok(())
    }
    fn parse_input(
        buf: &mut BufReader<I>,
        output_path: Option<String>,
        targets: Vec<String>,
        mutators: &[ParamMutatorFn],
    ) -> Result<Input, io::Error> {
        let source = W::decode_source(buf)?;
        let mut input = Input::new(source, output_path, targets);
        input.mutate(mutators);
        Ok(input)
    }
    pub fn parameters_mutator(&mut self, mutator: impl FnMut(&mut Parameters) + 'static) {
        self.param_mutators.push(Rc::new(RefCell::new(mutator)));
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::{env, fs};

    #[test]
    fn test_new_standalone_generator() {
        println!("{}", file!());
        println!("{}", env::current_dir().unwrap().display());
        let input = fs::File::open(
            env::current_dir()
                .unwrap()
                .join("tests/proto_op/kitchen.bin"),
        )
        .unwrap();
        let mut gen = Generator::new_standalone(input, &["kitchen.proto"], "").unwrap();
        gen.render().unwrap();
    }
}
