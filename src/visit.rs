#![allow(unused_variables)]

use crate::field::{
    EmbedField, EnumField, MapField, MappedEmbedField, MappedEnumField, MappedScalarField,
    OneofEmbedField, OneofEnumField, OneofField, OneofScalarField, RepeatedEmbedField,
    RepeatedEnumField, RepeatedField, RepeatedScalarField, ScalarField,
};
use crate::{
    Enum, EnumValue, Extension, Field, File, Message, Method, Node, Oneof, Package, Service,
};

pub trait Visitor<'a, U>: Sized {
    type Error;

    fn visit_node(&mut self, node: Node<'a, U>) -> Result<(), Self::Error> {
        visit_node(self, node)
    }

    fn visit_package(&mut self, pkg: Package<'a, U>) -> Result<(), Self::Error> {
        visit_package(self, pkg)
    }

    fn visit_file(&mut self, f: File<'a, U>) -> Result<(), Self::Error> {
        visit_file(self, f)
    }

    fn visit_message(&mut self, msg: Message<'a, U>) -> Result<(), Self::Error> {
        visit_message(self, msg)
    }

    fn visit_enum(&mut self, enm: Enum<'a, U>) -> Result<(), Self::Error> {
        visit_enum(self, enm)
    }

    fn visit_enum_value(&mut self, val: EnumValue<'a, U>) -> Result<(), Self::Error> {
        visit_enum_value(self, val)
    }

    fn visit_extension(&mut self, ext: Extension<'a, U>) -> Result<(), Self::Error> {
        visit_extension(self, ext)
    }

    fn visit_oneof(&mut self, oneof: Oneof<'a, U>) -> Result<(), Self::Error> {
        visit_oneof(self, oneof)
    }

    fn visit_real_oneof(&mut self, oneof: Oneof<'a, U>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_synthetic_oneof(&mut self, oneof: Oneof<'a, U>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_service(&mut self, svc: Service<'a, U>) -> Result<(), Self::Error> {
        visit_service(self, svc)
    }

    fn visit_method(&mut self, mth: Method<'a, U>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_field(&mut self, fld: Field<'a, U>) -> Result<(), Self::Error> {
        visit_field(self, fld)
    }

    fn visit_scalar_field(&mut self, fld: ScalarField<'a, U>) -> Result<(), Self::Error> {
        Ok(())
    }
    fn visit_embed_field(&mut self, fld: EmbedField<'a, U>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_enum_field(&mut self, fld: EnumField<'a, U>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_map_field(&mut self, fld: MapField<'a, U>) -> Result<(), Self::Error> {
        visit_map_field(self, fld)
    }

    fn visit_mapped_scalar_field(
        &mut self,
        fld: MappedScalarField<'a, U>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_mapped_enum_field(&mut self, fld: MappedEnumField<'a, U>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_mapped_embed_field(
        &mut self,
        fld: MappedEmbedField<'a, U>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_repeated_field(&mut self, fld: RepeatedField<'a, U>) -> Result<(), Self::Error> {
        visit_repeated_field(self, fld)
    }

    fn visit_repeated_scalar_field(
        &mut self,
        fld: RepeatedScalarField<'a, U>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_repeated_enum_field(
        &mut self,
        fld: RepeatedEnumField<'a, U>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_repeated_embed_field(
        &mut self,
        fld: RepeatedEmbedField<'a, U>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_oneof_field(&mut self, fld: OneofField<'a, U>) -> Result<(), Self::Error> {
        visit_oneof_field(self, fld)
    }
    fn visit_oneof_scalar_field(
        &mut self,
        fld: OneofScalarField<'a, U>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
    fn visit_oneof_embed_field(&mut self, fld: OneofEmbedField<'a, U>) -> Result<(), Self::Error> {
        Ok(())
    }
    fn visit_oneof_enum_field(&mut self, fld: OneofEnumField<'a, U>) -> Result<(), Self::Error> {
        Ok(())
    }
    fn visit_real_oneof_field(&mut self, fld: OneofField<'a, U>) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub fn visit_node<'a, U, V>(v: &mut V, node: Node<'a, U>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
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

pub fn visit_package<'a, U, V>(v: &mut V, pkg: Package<'a, U>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    for file in pkg.files() {
        v.visit_node(file.into())?;
    }
    Ok(())
}

pub fn visit_file<'a, U, V>(v: &mut V, f: File<'a, U>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    for msg in f.messages() {
        v.visit_node(msg.into())?;
    }
    for enm in f.enums() {
        v.visit_node(enm.into())?;
    }
    for svc in f.services() {
        v.visit_node(svc.into())?;
    }
    for ext in f.defined_extensions() {
        v.visit_node(ext.into())?;
    }
    Ok(())
}

pub fn visit_message<'a, U, V>(v: &mut V, msg: Message<'a, U>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    for n in msg.nodes() {
        visit_node(v, n)?;
    }
    Ok(())
}

pub fn visit_enum_value<'a, U, V>(v: &mut V, enum_value: EnumValue<'a, U>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    for n in enum_value.nodes() {
        v.visit_node(n.into())?;
    }
    Ok(())
}

pub fn visit_service<'a, U, V>(v: &mut V, svc: Service<'a, U>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    for n in svc.nodes() {
        v.visit_node(n)?;
    }
    Ok(())
}

pub fn visit_enum<'a, U, V>(v: &mut V, enm: Enum<'a, U>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    for n in enm.nodes() {
        v.visit_node(n)?;
    }

    Ok(())
}

pub fn visit_oneof<'a, U, V>(v: &mut V, one: Oneof<'a, U>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    if one.is_real() {
        v.visit_real_oneof(one)?;
    } else {
        v.visit_synthetic_oneof(one)?;
    }
    Ok(())
}

pub fn visit_field<'a, U, V>(v: &mut V, fld: Field<'a, U>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
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

pub fn visit_map_field<'a, U, V>(v: &mut V, fld: MapField<'a, U>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    match fld {
        MapField::Scalar(f) => v.visit_mapped_scalar_field(f),
        MapField::Enum(f) => v.visit_mapped_enum_field(f),
        MapField::Embed(f) => v.visit_mapped_embed_field(f),
    }
}
pub fn visit_repeated_field<'a, U, V>(v: &mut V, fld: RepeatedField<'a, U>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    match fld {
        RepeatedField::Scalar(f) => v.visit_repeated_scalar_field(f),
        RepeatedField::Enum(f) => v.visit_repeated_enum_field(f),
        RepeatedField::Embed(f) => v.visit_repeated_embed_field(f),
    }
}

pub fn visit_oneof_field<'a, U, V>(v: &mut V, fld: OneofField<'a, U>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    match fld {
        OneofField::Scalar(f) => v.visit_oneof_scalar_field(f),
        OneofField::Enum(f) => v.visit_oneof_enum_field(f),
        OneofField::Embed(f) => v.visit_oneof_embed_field(f),
    }
}

pub fn visit_real_oneof_field<'a, U, V>(v: &mut V, fld: OneofField<'a, U>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    for n in fld.nodes() {
        v.visit_node(n)?;
    }
    Ok(())
}

pub fn visit_synthetic_oneof_field<'a, U, V>(
    v: &mut V,
    fld: OneofField<'a, U>,
) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
    U: 'a,
{
    for n in fld.nodes() {
        v.visit_node(n)?;
    }
    Ok(())
}

pub fn visit_extension<'a, U, V>(v: &mut V, ext: Extension<'a, U>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    for n in ext.nodes() {
        v.visit_node(n)?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn example() {
        let p = File::default();
        for n in p.nodes() {}
    }
}
