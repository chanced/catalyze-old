#![allow(unused_variables)]

use crate::field::{
    EmbedField, EnumField, MapField, MappedEmbedField, MappedEnumField, MappedScalarField,
    OneofEmbedField, OneofEnumField, OneofField, OneofScalarField, RepeatedEmbedField,
    RepeatedEnumField, RepeatedField, RepeatedScalarField, ScalarField,
};
use crate::{
    Enum, EnumValue, Extension, Field, File, Message, Method, Node, Oneof, Package, Service,
};

pub trait Visitor: Sized {
    type Error;

    fn visit_node(&mut self, node: Node) -> Result<(), Self::Error> {
        visit_node(self, node)
    }

    fn visit_package(&mut self, pkg: Package) -> Result<(), Self::Error> {
        visit_package(self, pkg)
    }

    fn visit_file(&mut self, f: File) -> Result<(), Self::Error> {
        visit_file(self, f)
    }

    fn visit_message(&mut self, msg: Message) -> Result<(), Self::Error> {
        visit_message(self, msg)
    }

    fn visit_enum(&mut self, enm: Enum) -> Result<(), Self::Error> {
        visit_enum(self, enm)
    }

    fn visit_enum_value(&mut self, val: EnumValue) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_extension(&mut self, ext: Extension) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_oneof(&mut self, oneof: Oneof) -> Result<(), Self::Error> {
        visit_oneof(self, oneof)
    }

    fn visit_real_oneof(&mut self, oneof: Oneof) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_synthetic_oneof(&mut self, oneof: Oneof) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_service(&mut self, svc: Service) -> Result<(), Self::Error> {
        visit_service(self, svc)
    }

    fn visit_method(&mut self, mth: Method) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_field(&mut self, fld: Field) -> Result<(), Self::Error> {
        visit_field(self, fld)
    }

    fn visit_scalar_field(&mut self, fld: ScalarField) -> Result<(), Self::Error> {
        Ok(())
    }
    fn visit_embed_field(&mut self, fld: EmbedField) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_enum_field(&mut self, fld: EnumField) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_map_field(&mut self, fld: MapField) -> Result<(), Self::Error> {
        visit_map_field(self, fld)
    }

    fn visit_mapped_scalar_field(&mut self, fld: MappedScalarField) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_mapped_enum_field(&mut self, fld: MappedEnumField) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_mapped_embed_field(&mut self, fld: MappedEmbedField) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_repeated_field(&mut self, fld: RepeatedField) -> Result<(), Self::Error> {
        visit_repeated_field(self, fld)
    }

    fn visit_repeated_scalar_field(&mut self, fld: RepeatedScalarField) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_repeated_enum_field(&mut self, fld: RepeatedEnumField) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_repeated_embed_field(&mut self, fld: RepeatedEmbedField) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_oneof_field(&mut self, fld: OneofField) -> Result<(), Self::Error> {
        visit_oneof_field(self, fld)
    }

    fn visit_real_oneof_field(&mut self, fld: OneofField) -> Result<(), Self::Error> {
        visit_real_oneof_field(self, fld)
    }

    fn visit_real_oneof_scalar_field(&mut self, fld: OneofScalarField) -> Result<(), Self::Error> {
        Ok(())
    }
    fn visit_real_oneof_enum_field(&mut self, fld: OneofEnumField) -> Result<(), Self::Error> {
        Ok(())
    }
    fn visit_real_oneof_embed_field(&mut self, fld: OneofEmbedField) -> Result<(), Self::Error> {
        Ok(())
    }
    fn visit_synthetic_oneof_field(&mut self, fld: OneofField) -> Result<(), Self::Error> {
        visit_synthetic_oneof_field(self, fld)
    }
    fn visit_synthetic_oneof_scalar_field(
        &mut self,
        fld: OneofScalarField,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
    fn visit_synthetic_oneof_enum_field(&mut self, fld: OneofEnumField) -> Result<(), Self::Error> {
        Ok(())
    }
    fn visit_synthetic_oneof_embed_field(
        &mut self,
        fld: OneofEmbedField,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub fn visit_node<'a, V>(v: &mut V, node: Node) -> Result<(), V::Error>
where
    V: Visitor,
{
    match node {
        Node::Package(p) => v.visit_package(p),
        Node::File(f) => v.visit_file(f),
        Node::Message(m) => v.visit_message(m),
        Node::Oneof(o) => v.visit_oneof(o),
        Node::Enum(e) => v.visit_enum(e),
        Node::EnumValue(ev) => v.visit_enum_value(ev),
        Node::Service(s) => v.visit_service(s),
        Node::Method(m) => v.visit_method(m),
        Node::Field(f) => v.visit_field(f),
        Node::Extension(e) => v.visit_extension(e),
    }
}

pub fn visit_package<'a, V>(v: &mut V, pkg: Package) -> Result<(), V::Error>
where
    V: Visitor,
{
    for file in pkg.files() {
        v.visit_node(file.into())?;
    }
    Ok(())
}

pub fn visit_file<'a, V>(v: &mut V, f: File) -> Result<(), V::Error>
where
    V: Visitor,
{
    for node in f.nodes() {
        v.visit_node(node)?;
    }
    // for msg in f.messages() {
    //     v.visit_node(msg.into())?;
    // }
    // for enm in f.enums() {
    //     v.visit_node(enm.into())?;
    // }
    // for svc in f.services() {
    //     v.visit_node(svc.into())?;
    // }
    // for ext in f.defined_extensions() {
    //     v.visit_node(ext.into())?;
    // }
    Ok(())
}

pub fn visit_message<'a, V>(v: &mut V, msg: Message) -> Result<(), V::Error>
where
    V: Visitor,
{
    for n in msg.nodes() {
        v.visit_node(n)?;
    }
    // for ext in msg.defined_extensions() {
    //     v.visit_node(ext.into())?;
    // }
    // for enm in msg.enums() {
    //     v.visit_node(enm.into())?;
    // }
    // for fld in msg.fields() {
    //     v.visit_node(fld.into())?;
    // }
    // for one in msg.oneofs() {
    //     v.visit_node(one.into())?;
    // }
    // for emb in msg.messages() {
    //     v.visit_node(emb.into())?;
    // }
    Ok(())
}

pub fn visit_service<'a, V>(v: &mut V, svc: Service) -> Result<(), V::Error>
where
    V: Visitor,
{
    for mth in svc.methods() {
        v.visit_node(mth.into())?;
    }
    Ok(())
}

pub fn visit_enum<'a, V>(v: &mut V, enm: Enum) -> Result<(), V::Error>
where
    V: Visitor,
{
    for node in enm.nodes() {
        v.visit_node(node)?;
    }
    // for ev in enm.values() {
    //     v.visit_node(ev.into())?;
    // }

    Ok(())
}

pub fn visit_oneof<'a, V>(v: &mut V, one: Oneof) -> Result<(), V::Error>
where
    V: Visitor,
{
    if one.is_real() {
        v.visit_real_oneof(one)?;
    } else {
        v.visit_synthetic_oneof(one)?;
    }
    Ok(())
}

pub fn visit_field<'a, V>(v: &mut V, fld: Field) -> Result<(), V::Error>
where
    V: Visitor,
{
    match fld {
        Field::Embed(f) => v.visit_embed_field(f),
        Field::Enum(f) => v.visit_enum_field(f),
        Field::Map(f) => v.visit_map_field(f),
        Field::Oneof(f) => v.visit_oneof_field(f),
        Field::Repeated(f) => v.visit_repeated_field(f),
        Field::Scalar(f) => v.visit_scalar_field(f),
    }
}

pub fn visit_map_field<'a, V>(v: &mut V, fld: MapField) -> Result<(), V::Error>
where
    V: Visitor,
{
    match fld {
        MapField::Scalar(f) => v.visit_mapped_scalar_field(f),
        MapField::Enum(f) => v.visit_mapped_enum_field(f),
        MapField::Embed(f) => v.visit_mapped_embed_field(f),
    }
}
pub fn visit_repeated_field<'a, V>(v: &mut V, fld: RepeatedField) -> Result<(), V::Error>
where
    V: Visitor,
{
    match fld {
        RepeatedField::Scalar(f) => v.visit_repeated_scalar_field(f),
        RepeatedField::Enum(f) => v.visit_repeated_enum_field(f),
        RepeatedField::Embed(f) => v.visit_repeated_embed_field(f),
    }
}
pub fn visit_oneof_field<'a, V>(v: &mut V, fld: OneofField) -> Result<(), V::Error>
where
    V: Visitor,
{
    if fld.is_in_real_oneof() {
        v.visit_real_oneof_field(fld)
    } else {
        v.visit_synthetic_oneof_field(fld)
    }
}

pub fn visit_real_oneof_field<'a, V>(v: &mut V, fld: OneofField) -> Result<(), V::Error>
where
    V: Visitor,
{
    match fld {
        OneofField::Scalar(f) => v.visit_real_oneof_scalar_field(f),
        OneofField::Enum(f) => v.visit_real_oneof_enum_field(f),
        OneofField::Embed(f) => v.visit_real_oneof_embed_field(f),
    }
}
pub fn visit_synthetic_oneof_field<'a, V>(v: &mut V, fld: OneofField) -> Result<(), V::Error>
where
    V: Visitor,
{
    match fld {
        OneofField::Scalar(f) => v.visit_synthetic_oneof_scalar_field(f),
        OneofField::Enum(f) => v.visit_synthetic_oneof_enum_field(f),
        OneofField::Embed(f) => v.visit_synthetic_oneof_embed_field(f),
    }
}

// #[cfg(test)]
// mod test {
//     use crate::*;
//     #[test]
//     fn example() {
//         let p = File::default();
//         let m = Message::default();
//         for n in p.nodes() {}
//     }
// }
