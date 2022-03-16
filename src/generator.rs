use crate::Module;
use anyhow::{bail, Error};
use bytes::Buf;
use prost::Message;

use std::{
    collections::HashMap,
    fs::File,
    io::{self, stdin},
    io::{stdout, BufReader, BufWriter, Cursor, Read, Stdin, Stdout, Write},
    ops::Index,
    path::{Path, PathBuf},
};

const OUTPUT_PATH_KEY: &str = "output_path";

#[derive(Clone, Debug)]
pub struct Parameters {
    table: HashMap<String, String>,
}
impl Parameters {
    pub fn new(params: &str) -> Self {
        let table = Parameters::parse_table(params);
        let output_path = table.get(OUTPUT_PATH_KEY).cloned();
        Self { table }
    }
    pub fn output_path(&self) -> Option<String> {
        self.get(OUTPUT_PATH_KEY)
    }
    pub fn get(&self, key: &str) -> Option<String> {
        self.table.get(key).cloned()
    }
    pub fn len(&self) -> usize {
        self.table.len()
    }
    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }
    pub fn contains_key(&self, key: &str) -> bool {
        self.table.contains_key(key)
    }
    pub fn iter(&self) -> std::collections::hash_map::Iter<String, String> {
        self.table.iter()
    }
    pub fn set_output_path(&self, path: &str) {
        self.table
            .insert(OUTPUT_PATH_KEY.to_string(), path.to_string());
    }

    pub fn insert(&mut self, path: &str) {
        self.table.insert(path.to_string(), path.to_string());
    }

    fn parse_table(val: &str) -> HashMap<String, String> {
        let mut params = HashMap::new();
        for param in val.split(",") {
            if param.contains('=') {
                let parts = param.splitn(2, '=').collect::<Vec<_>>();
                params.insert(parts[0].to_string(), parts[1].to_string());
            } else {
                params.insert(param.to_string(), "".to_string());
            }
        }
        params
    }
}
impl Index<String> for Parameters {
    type Output = String;
    fn index(&self, key: String) -> &Self::Output {
        self.table.get(&key).unwrap_or(&"".to_string())
    }
}
impl Default for Parameters {
    fn default() -> Self {
        Self {
            table: HashMap::new(),
        }
    }
}

impl From<String> for Parameters {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}
impl From<&str> for Parameters {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

pub struct Input<'a> {
    pub files: &'a [prost_types::FileDescriptorProto],
    pub targets: Vec<String>,
    pub parmeters: Parameters,
    pub protoc_version: Option<semver::Version>,
}
impl<'a> Input<'a> {
    pub fn new(files: &'a [prost_types::FileDescriptorProto], params: &str) -> Self {
        let parmeters = Parameters::new(params);
        Self {
            files,
            targets: vec![],
            parmeters,
            protoc_version: None,
        }
    }
    pub fn files(&self) -> std::slice::Iter<'a, prost_types::FileDescriptorProto> {
        self.files.iter()
    }
}
pub trait Workflow: Sized {
    fn read_input<I: Read, O: Write>(
        g: &mut Generator<Self, I, O>,
    ) -> Result<&mut Generator<Self, I, O>, io::Error>;
}

/// The `Standalone` workflow reads a `FileDescriptorSet`, typically from the
/// contents of a saved output from `protoc`, and generates based on a list of
/// target (proto) files. The output is saved to disk at the specified output
/// path.
pub struct Standalone {}
impl Workflow for Standalone {
    fn read_input<I, O>(
        g: &mut Generator<Self, I, O>,
    ) -> Result<&mut Generator<Self, I, O>, io::Error>
    where
        I: Read,
        O: Write,
    {
        let mut buf = Vec::new();
        g.input.read_to_end(&mut buf)?;
        let buf = Cursor::new(buf);
        let fds = prost_types::FileDescriptorSet::decode(buf)?;
        g.file_desc_set = Some(fds);

        Ok(g)
    }
}

/// ProtocPlugins reads a `CodeGeneratorRequest` from `stdin` and writes a
/// `CodeGeneratorResponse` to `stdout`.
///
/// The input (`stdin`) and output (`stdout`) can be configured, using any
/// reader and writer respectively.
pub struct ProtocPlugin {}
impl Workflow for ProtocPlugin {
    fn read_input<I, O>(
        g: &mut Generator<Self, I, O>,
    ) -> Result<&mut Generator<Self, I, O>, io::Error>
    where
        I: Read,
        O: Write,
    {
        let mut buf = Vec::new();
        g.input.read_to_end(&mut buf)?;
        let buf = Cursor::new(buf);
        let cgr = prost_types::compiler::CodeGeneratorRequest::decode(buf)?;
        g.code_gen_req = Some(cgr);
        // g.parsed_input = Some(input);
        todo!()
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
    output_dir: Option<PathBuf>,
    modules: Vec<Module>,
    targets: Vec<String>,
    workflow: Box<W>,
    file_desc_set: Option<prost_types::FileDescriptorSet>,
    code_gen_req: Option<prost_types::compiler::CodeGeneratorRequest>,
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
            output_dir: None,
            file_desc_set: None,
            code_gen_req: None,
        }
    }
}

impl<I, O> Generator<ProtocPlugin, I, O>
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
            output_dir: self.output_dir,
            output: self.output,
            file_desc_set: self.file_desc_set,
            code_gen_req: self.code_gen_req,
        }
    }
    pub fn output<W: Write>(self, output: W) -> Generator<ProtocPlugin, I, W> {
        Generator {
            input: self.input,
            modules: self.modules,
            targets: self.targets,
            workflow: self.workflow,
            output_dir: self.output_dir,
            output: Some(BufWriter::new(output)),
            file_desc_set: self.file_desc_set,
            code_gen_req: self.code_gen_req,
        }
    }
}

impl<W: Workflow, I: Read> Generator<W, I> {
    /// Returns a new `Generator` with the `Standalone` workflow.
    pub fn new_stand_alone(
        input: I,
        target_protos: &[impl AsRef<Path>],
        output_dir: impl AsRef<Path>,
    ) -> Result<Generator<Standalone, I, File>, anyhow::Error> {
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
            workflow: Box::new(Standalone {}),
            output_dir: Some(output_dir.as_ref().to_path_buf()),
            output: None,
            file_desc_set: None,
            code_gen_req: None,
        })
    }
}

impl<W: Workflow, I: Read, O: Write> Generator<W, I, O> {
    fn parsed_input<'a>(&'a self) -> Result<Input<'a>, anyhow::Error> {
        if let Some(cgr) = self.code_gen_req {
            let mut input = Input::new()
            Ok(&input)
        } else if let Some(fds) = self.file_desc_set {
            let mut input = Input::new();
            for file in fds.file {
                input.files.push(File::from_proto(file));
            }
            Ok(&input)
        } else {
            bail!("no input provided");
        }
    }
}
