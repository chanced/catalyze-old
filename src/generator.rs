use crate::{Ast, Input, ParamMutatorFn, Parameters, Source};
use anyhow::{anyhow, bail};
use prost::Message;

use std::{
    cell::RefCell,
    error::Error,
    fs,
    io::{self, stdin},
    io::{stdout, BufReader, BufWriter, Cursor, Read, Stdin, Stdout, Write},
    path::Path,
    rc::Rc,
};

use std::collections::HashMap;

use crate::{Artifact, Context, File};

pub trait Module {
    fn name(&self) -> &'static str;
    fn init(&mut self, ctx: Context);
    fn execute(
        &mut self,
        targets: HashMap<String, File>,
        ast: Ast,
    ) -> Result<Vec<Artifact>, Box<dyn Error + Send + Sync + 'static>>;
}

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
    modules: Vec<Box<dyn Module>>,
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
        let path = self
            .output_path
            .as_ref()
            .map_or(Path::new("."), Path::new)
            .to_owned();

        for m in self.modules.iter_mut() {
            m.init(Context::new(
                path.clone(),
                self.parsed_input.parameters().clone(),
            ));
        }

        let ast = Ast::new(&self.parsed_input)?;

        let mut artifacts = vec![];
        for m in self.modules.iter_mut() {
            let mut res = m
                .execute(ast.target_file_map(), ast.clone())
                .map_err(|e| anyhow!(e).context(format!("module {}", m.name())))?;
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
    pub fn module(&mut self, module: impl Module + 'static) -> &mut Self {
        self.modules.push(Box::new(module));
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::{collections::HashMap, env, fs};

    struct Mod {}
    impl Module for Mod {
        fn name(&self) -> &'static str {
            "mod"
        }

        fn init(&mut self, ctx: Context) {
            ctx.parameters().set_param("", "value");
        }

        fn execute(
            &mut self,
            targets: HashMap<String, File>,
            ast: Ast,
        ) -> Result<Vec<Artifact>, Box<dyn std::error::Error + Send + Sync + 'static>> {
            assert_eq!(targets.len(), 1);
            assert!(targets.contains_key("kitchen/kitchen.proto"));
            let sink = ast.node(".kitchen.Sink").unwrap();
            let fields = sink.as_message().unwrap().fields();
            let brand = fields.get(0).unwrap();
            assert_eq!(brand.name(), "brand");
            assert!(brand.is_enum(), "brand is not enum");

            let sink_proto = ast.file("kitchen/sink.proto").unwrap();

            let kitchen_proto = ast.file("kitchen/kitchen.proto").unwrap();
            let kitchen = kitchen_proto.message("Kitchen").expect("Kitchen not found");
            let color = kitchen_proto.message("Color").expect("Color not found");

            let style_field = kitchen.field("style").expect("style not found");
            assert_eq!(style_field.name(), "style");
            match style_field {
                Field::Enum(e) => {
                    let style = e.enumeration();
                    assert_eq!(style.name(), "Style");
                }
                _ => panic!("style should be an enum"),
            }

            let sink_field = kitchen.field("sink").expect("sink not found");
            assert_eq!(sink_field.name(), "sink");
            assert!(!sink_field.is_well_known_type());
            assert!(sink_field.is_embed());

            let dish_counts_field = kitchen.field("dish_counts").expect("dish_counts not found");
            assert!(dish_counts_field.is_map());
            assert_eq!(dish_counts_field.name(), "dish_counts");

            let utensils_field = kitchen.field("utensils").expect("utensils not found");
            assert_eq!("utensils", utensils_field.name());
            assert!(utensils_field.is_repeated());
            assert!(utensils_field.is_scalar());

            match utensils_field {
                Field::Repeated(utensils_field) => {
                    assert_eq!(utensils_field.name(), "utensils");
                    assert!(utensils_field.is_scalar());
                    match utensils_field {
                        RepeatedField::Scalar(utensils_field) => {
                            assert_eq!(utensils_field.scalar(), Scalar::String);
                        }
                        _ => panic!("utensils should be a repeated scalar field"),
                    }
                }
                _ => panic!("utensils should be a repeated field"),
            }

            match dish_counts_field {
                Field::Map(dish_counts) => match dish_counts {
                    MapField::Scalar(scf) => {
                        assert_eq!(scf.name(), "dish_counts");
                        assert_eq!(scf.key(), Key::String);
                        assert_eq!(scf.scalar(), Scalar::Uint32);
                    }
                    _ => panic!("map field should be scalar"),
                },
                _ => panic!("dish_counts should be a map"),
            }

            let wall_colors_field = kitchen.field("wall_colors").expect("wall_colors not found");
            match wall_colors_field {
                Field::Repeated(wall_colors_field) => match wall_colors_field {
                    RepeatedField::Embed(wall_colors_field) => {
                        let color_embed = wall_colors_field.embed();
                        assert_eq!(color_embed, color);
                        assert_eq!(color_embed.name(), "Color");
                    }
                    _ => panic!("wall_colors should be a repeated embed field"),
                },
                _ => panic!("wall_colors should be repeated"),
            }
            let appliance_colors = kitchen
                .field("appliance_colors")
                .expect("appliance_colors not found");

            match appliance_colors {
                Field::Map(appliance_colors) => match appliance_colors {
                    MapField::Embed(appliance_colors) => {
                        let color_embed = appliance_colors.embed();
                        assert_eq!(color_embed, color);
                        assert_eq!(color_embed.name(), "Color");
                    }
                    _ => panic!("appliance_colors should be a mapped embed field"),
                },
                _ => panic!("appliance_colors should be a map"),
            }

            let sink = sink_proto.message("Sink").expect("Sink not found");

            let brand_field = sink.field("brand").expect("brand not found");

            assert!(brand_field.is_enum());
            assert_eq!(brand_field.name(), "brand");
            match brand_field {
                Field::Enum(brand_field) => {
                    let brand = brand_field.enumeration();
                    assert_eq!(brand.name(), "Brand");
                }
                _ => panic!("brand should be an enum"),
            }

            Ok(vec![])
        }
    }

    #[test]
    fn test_new_standalone_generator() {
        let input = fs::File::open(
            env::current_dir()
                .unwrap()
                .join("tests/proto_op/kitchen.bin"),
        )
        .unwrap();
        let mut gen = Generator::new_standalone(input, &["kitchen/kitchen.proto"], "").unwrap();
        gen.module(Mod {}).render().unwrap();
    }
}
