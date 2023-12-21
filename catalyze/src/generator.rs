use crate::Ast;
use crate::{Artifact, File};
use protobuf::Message;
use std::collections::HashMap;
use std::{
    fs,
    io::{self},
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
    path::PathBuf,
};

// impl Workflow for Standalone {
//     fn decode_source<I: Read>(input: &mut BufReader<I>) -> Result<Source, io::Error> {
//         let mut buf = Vec::new();
//         input.read_to_end(&mut buf)?;
//         let fds = protobuf::descriptor::FileDescriptorSet::parse_from_bytes(&buf)?;
//         Ok(Source::FileDescriptorSet(fds))
//     }
// }

// impl Workflow for ProtocPlugin {
//     fn decode_source<I: Read>(input: &mut BufReader<I>) -> Result<Source, io::Error> {
//         let mut buf = Vec::new();
//         input.read_to_end(&mut buf)?;
//         let cgr = protobuf::plugin::CodeGeneratorRequest::parse_from_bytes(&buf)?;
//         Ok(Source::CodeGeneratorRequest(cgr))
//     }
// }

// impl<I, O> Generator<ProtocPlugin, I, O>
// where
//     I: Read,
//     O: Write,
// {
//     pub fn input<R: Read>(self, input: R) -> Generator<ProtocPlugin, R, O> {
//         Generator {
//             input: BufReader::new(input),
//             modules: self.modules,
//             targets: self.targets,
//             workflow: self.workflow,
//             output_path: self.output_path,
//             output: self.output,
//             parsed_input: self.parsed_input,
//             param_mutators: self.param_mutators,
//         }
//     }
//     pub fn output<W: Write>(self, output: W) -> Generator<ProtocPlugin, I, W> {
//         Generator {
//             input: self.input,
//             modules: self.modules,
//             targets: self.targets,
//             workflow: self.workflow,
//             output_path: self.output_path,
//             output: Some(BufWriter::new(output)),
//             parsed_input: self.parsed_input,
//             param_mutators: self.param_mutators,
//         }
//     }
// }

// impl<I> Generator<Standalone, I>
// where
//     I: Read,
// {
//     /// Returns a new `Generator` with the `Standalone` workflow.
//     pub fn new_standalone<'a, T, O>(
//         input: I,
//         target_protos: T,
//         output_path: O,
//     ) -> Generator<Standalone, I, fs::File>
//     where
//         T: IntoIterator,
//         T::Item: AsRef<Path>,
//         O: AsRef<Path>,
//     {
//         Generator {
//             targets: target_protos
//                 .into_iter()
//                 .map(|p| p.as_ref().to_path_buf())
//                 .collect(),
//             input: BufReader::new(input),
//             modules: Vec::new(),
//             workflow: Standalone,
//             output_path: Some(output_path.as_ref().to_path_buf()),
//             output: None,
//             parsed_input: Default::default(),
//             param_mutators: Vec::new(),
//         }
//     }
// }

// impl<'a, W, I, O> Generator<W, I, O>
// where
//     W: Workflow,
//     I: Read,
//     O: Write,
// {
//     pub fn render(mut self) -> Result<(), crate::error::Error> {
//         let input = Self::parse_input(
//             &mut self.input,
//             self.output_path.clone(),
//             self.targets.clone(),
//             self.param_mutators.as_slice(),
//         )?;
//         self.parsed_input = input;
//         let path = self
//             .output_path
//             .as_ref()
//             .map_or(Path::new("."), Path::new)
//             .to_owned();
//         let ast = Ast::new(&self.parsed_input)?;
//         let mut artifacts = vec![];

//         for m in self.modules.iter_mut() {
//             let mut res = m.execute(ast.target_file_map(), ast.clone())?;
//             artifacts.append(&mut res);
//         }
//         Ok(())
//     }
//     fn parse_input(
//         buf: &mut BufReader<I>,
//         output_path: Option<String>,
//         targets: Vec<String>,
//     ) -> Result<Input, io::Error> {
//         let source = W::decode_source(buf)?;
//         let mut input = Input::new(source, output_path, targets);
//         input.mutate(mutators);
//         Ok(input)
//     }
//     pub fn module(&mut self, module: impl Module + 'static) -> &mut Self {
//         self.modules.push(Box::new(module));
//         self
//     }
// }

// #[cfg(test)]
// mod tests {

//     use crate::*;
//     use std::{collections::HashMap, env, fs, io::Cursor};

//     struct Mod;
//     impl Module for Mod {
//         fn name(&self) -> &'static str {
//             "mod"
//         }

//         fn execute(
//             &mut self,
//             targets: HashMap<String, File>,
//             ast: Ast,
//         ) -> Result<Vec<Artifact>, Box<dyn std::error::Error + Send + Sync + 'static>> {
//             assert_eq!(targets.len(), 1);
//             assert!(targets.contains_key("kitchen/kitchen.proto"));
//             let sink = ast.node(".kitchen.Sink").unwrap();
//             assert!(matches!(sink, Node::Message(_)));
//             let sink_proto = ast.file("kitchen/sink.proto").unwrap();

//             let kitchen_proto = ast.file("kitchen/kitchen.proto").unwrap();
//             let kitchen = kitchen_proto.message("Kitchen").expect("Kitchen not found");
//             let color = kitchen_proto.message("Color").expect("Color not found");

//             let style_field = kitchen.field("style").expect("style not found");
//             assert_eq!(style_field.name(), "style");
//             match style_field {
//                 Field::Enum(e) => {
//                     let style = e.enum_();
//                     assert_eq!(style.name(), "Style");
//                 }
//                 _ => panic!("style should be an enum"),
//             }

//             let sink_field = kitchen.field("sink").expect("sink not found");
//             assert_eq!(sink_field.name(), "sink");
//             assert!(!sink_field.is_well_known_type());
//             assert!(sink_field.is_embed());

//             let dish_counts_field = kitchen.field("dish_counts").expect("dish_counts not found");
//             assert!(dish_counts_field.is_map());
//             assert_eq!(dish_counts_field.name(), "dish_counts");

//             let utensils_field = kitchen.field("utensils").expect("utensils not found");
//             assert_eq!("utensils", utensils_field.name());
//             assert!(utensils_field.is_repeated());
//             assert!(utensils_field.is_scalar());

//             match utensils_field {
//                 Field::Repeated(utensils_field) => {
//                     assert_eq!(utensils_field.name(), "utensils");
//                     assert!(utensils_field.is_scalar());
//                     match utensils_field {
//                         RepeatedField::Scalar(utensils_field) => {
//                             assert_eq!(utensils_field.scalar(), Scalar::String);
//                         }
//                         _ => panic!("utensils should be a repeated scalar field"),
//                     }
//                 }
//                 _ => panic!("utensils should be a repeated field"),
//             }

//             match dish_counts_field {
//                 Field::Map(dish_counts) => match dish_counts {
//                     MapField::Scalar(scf) => {
//                         assert_eq!(scf.name(), "dish_counts");
//                         assert_eq!(scf.key(), Key::String);
//                         assert_eq!(scf.scalar(), Scalar::Uint32);
//                     }
//                     _ => panic!("map field should be scalar"),
//                 },
//                 _ => panic!("dish_counts should be a map"),
//             }

//             let wall_colors_field = kitchen.field("wall_colors").expect("wall_colors not found");
//             match wall_colors_field {
//                 Field::Repeated(wall_colors_field) => match wall_colors_field {
//                     RepeatedField::Embed(wall_colors_field) => {
//                         let color_embed = wall_colors_field.embed();
//                         assert_eq!(color_embed, color);
//                         assert_eq!(color_embed.name(), "Color");
//                     }
//                     _ => panic!("wall_colors should be a repeated embed field"),
//                 },
//                 _ => panic!("wall_colors should be repeated"),
//             }
//             let appliance_colors = kitchen
//                 .field("appliance_colors")
//                 .expect("appliance_colors not found");

//             match appliance_colors {
//                 Field::Map(appliance_colors) => match appliance_colors {
//                     MapField::Embed(appliance_colors) => {
//                         let color_embed = appliance_colors.embed();
//                         assert_eq!(color_embed, color);
//                         assert_eq!(color_embed.name(), "Color");
//                     }
//                     _ => panic!("appliance_colors should be a mapped embed field"),
//                 },
//                 _ => panic!("appliance_colors should be a map"),
//             }

//             let sink = sink_proto.message("Sink").expect("Sink not found");
//             let brand = sink.enum_("Brand").expect("Brand not found");
//             let brand_field = sink.field("brand").expect("brand not found");

//             assert!(brand_field.is_enum());
//             assert_eq!(brand_field.name(), "brand");
//             match brand_field {
//                 Field::Enum(brand_field) => {
//                     assert_eq!(brand, brand_field.enum_());
//                     assert_eq!(brand_field.name(), "brand");
//                     assert_eq!(brand, brand_field.enum_());
//                 }
//                 _ => panic!("brand should be an enum"),
//             }

//             let material_field = sink.field("material").expect("material not found");

//             assert!(material_field.is_embed());
//             assert_eq!(material_field.name(), "material");

//             let material = sink_proto.message("Material").expect("Material not found");

//             match material_field {
//                 Field::Embed(material_field) => {
//                     let material_embed = material_field.embed();
//                     assert_eq!(material_embed, material);
//                     assert_eq!(material_embed.name(), "Material");
//                 }
//                 _ => panic!("material should be an embed field"),
//             }

//             let model_field = sink.field("model").expect("model not found");
//             assert_eq!(model_field.name(), "model");
//             assert!(model_field.is_scalar());
//             match model_field {
//                 Field::Scalar(model_field) => {
//                     assert_eq!(model_field.scalar(), Scalar::String);
//                 }
//                 _ => panic!("model should be a scalar field"),
//             }
//             let basin_count_field = sink.field("basin_count").expect("basin_count not found");
//             assert_eq!(basin_count_field.name(), "basin_count");
//             assert!(basin_count_field.is_scalar());
//             assert_eq!(basin_count_field.number(), 4);
//             match basin_count_field {
//                 Field::Scalar(basin_count_field) => {
//                     assert_eq!(basin_count_field.scalar(), Scalar::Uint32);
//                 }
//                 _ => panic!("basin_count should be a scalar field"),
//             }

//             let installed_field = sink.field("installed").expect("installed not found");

//             assert_eq!(installed_field.name(), "installed");

//             assert!(
//                 installed_field.is_embed(),
//                 "installed_field's google.protobuf.Timestamp should be an embedded message"
//             );
//             match installed_field {
//                 Field::Embed(installed_field) => {
//                     // println!("{:#?}", installed_field.embed().package());

//                     assert!(
//                         installed_field.is_well_known_type(),
//                         "installed_field is a google.protobuf.Timestamp which should be a well-known-type"
//                     );

//                     let wkt = installed_field
//                         .embed()
//                         .well_known_message()
//                         .expect("google.protobuf.Timestamp should be well-known");
//                     assert_eq!(wkt, WellKnownMessage::Timestamp);
//                 }
//                 _ => panic!("installed should be an embed field"),
//             }

//             let brand = sink.enum_("Brand").expect("Brand not found");
//             assert_eq!(brand.value("KRAUS").expect("KRAUS not found"), 0);
//             assert_eq!(brand.value("SWANSTONE").expect("SWANSTONE not found"), 1);
//             assert_eq!(brand.value("HOUZER").expect("HOUZER not found"), 2);
//             assert_eq!(brand.value("BLANCO").expect("BLANCO not found"), 3);
//             assert_eq!(brand.value("KOHLER").expect("KOHLER not found"), 4);

//             let cooking_service = kitchen_proto.service("Cooking").expect("Cooking not found");
//             let saute_method = cooking_service.method("Saute").expect("Saute not found");
//             let saute_request = kitchen_proto
//                 .message("SauteRequest")
//                 .expect("SauteRequest not found");
//             let saute_response = kitchen_proto
//                 .message("SauteResponse")
//                 .expect("SauteResponse not found");
//             assert_eq!(saute_method.name(), "Saute");
//             assert_eq!(saute_method.input(), saute_request);
//             assert_eq!(saute_method.output(), saute_response);
//             assert!(
//                 !saute_method.is_bidirectional_streaming(),
//                 "Saute should not be bidirectional streaming"
//             );
//             assert!(
//                 !saute_method.is_client_streaming(),
//                 "Saute should not be client streaming"
//             );
//             assert!(
//                 !saute_method.is_server_streaming(),
//                 "Saute should not be server streaming"
//             );
//             let ice_request = kitchen_proto
//                 .message("IceRequest")
//                 .expect("IceRequest not found");
//             let ice_response = kitchen_proto
//                 .message("IceResponse")
//                 .expect("IceResponse not found");

//             let dispense_ice_method = cooking_service
//                 .method("DispenseIce")
//                 .expect("DispenseIce not found");

//             assert_eq!(dispense_ice_method.input(), ice_request);
//             assert_eq!(dispense_ice_method.output(), ice_response);
//             assert!(
//                 !dispense_ice_method.is_client_streaming(),
//                 "DispenseIce should not be client streaming"
//             );
//             assert!(
//                 dispense_ice_method.is_server_streaming(),
//                 "DispenseIce should be server streaming"
//             );
//             assert!(
//                 !dispense_ice_method.is_bidirectional_streaming(),
//                 "DispenseIce should not be bidirectional streaming"
//             );

//             let load_fridge_method = cooking_service
//                 .method("LoadFridge")
//                 .expect("LoadFridge not found");

//             assert!(
//                 load_fridge_method.is_client_streaming(),
//                 "LoadFridge should be client streaming"
//             );
//             assert!(
//                 !load_fridge_method.is_server_streaming(),
//                 "LoadFridge should not be server streaming"
//             );

//             let order_drinks_method = cooking_service
//                 .method("OrderDrinks")
//                 .expect("OrderDrinks not found");
//             assert!(
//                 order_drinks_method.is_bidirectional_streaming(),
//                 "OrderDrinks should be bi-directional streaming"
//             );

//             Ok(vec![])
//         }
//     }

//     #[test]
//     fn test_new_standalone_generator() {
//         let input = fs::File::open(
//             env::current_dir()
//                 .unwrap()
//                 .join("tests/proto_op/kitchen.bin"),
//         )
//         .unwrap();
//         let mut gen = Generator::new_standalone(input, &["kitchen/kitchen.proto"], "").unwrap();
//         gen.module(Mod {}).render().unwrap();
//     }

//     #[test]
//     fn test_new_protoc_plugin_generator() {
//         use std::io::prelude::*;
//         let mut input_file = std::fs::File::open("tests/code-generator-requests/kitchen").unwrap();
//         let mut input = Vec::new();
//         input_file.read_to_end(&mut input).unwrap();
//         let mut input = Cursor::new(input);
//         let mut gen = Generator::new_protoc_plugin().input(&mut input);
//         gen.module(Mod {}).render().unwrap();
//     }
// }
