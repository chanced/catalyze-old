#![allow(unused_variables)]

use crate::field::{
    EmbedField, EnumField, MapField, MappedEmbedField, MappedEnumField, MappedScalarField,
    OneofEnumField, OneofField, OneofMessageField, OneofScalarField, RealOneofField,
    RepeatedEmbedField, RepeatedEnumField, RepeatedField, RepeatedScalarField, ScalarField,
    SyntheticOneofField,
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
        todo!()
    }

    fn visit_extension(&mut self, ext: Extension<'a, U>) -> Result<(), Self::Error> {
        todo!()
    }

    fn visit_oneof(&mut self, oneof: Oneof<'a, U>) -> Result<(), Self::Error> {
        todo!()
    }

    fn visit_real_oneof(&mut self, oneof: Oneof<'a, U>) -> Result<(), Self::Error> {
        todo!()
    }

    fn visit_synthetic_oneof(&mut self, oneof: Oneof<'a, U>) -> Result<(), Self::Error> {
        todo!()
    }

    fn visit_service(&mut self, svc: Service<'a, U>) -> Result<(), Self::Error> {
        todo!()
    }

    fn visit_method(&mut self, mtd: Method<'a, U>) -> Result<(), Self::Error> {
        todo!()
    }

    fn visit_field(&mut self, fld: Field<'a, U>) -> Result<(), Self::Error> {
        visit_node(self, fld.into())
    }

    fn visit_scalar_field(&mut self, fld: ScalarField<'a, U>) -> Result<(), Self::Error> {
        todo!()
    }
    fn visit_message_field(&mut self, fld: EmbedField<'a, U>) -> Result<(), Self::Error> {
        todo!()
    }

    fn visit_enum_field(&mut self, fld: EnumField<'a, U>) -> Result<(), Self::Error> {
        todo!()
    }

    fn visit_map_field(&mut self, fld: MapField<'a, U>) -> Result<(), Self::Error> {
        visit_map_field(self, fld)
    }

    fn visit_map_scalar_field(&mut self, fld: MappedScalarField<'a, U>) -> Result<(), Self::Error> {
        todo!()
    }

    fn visit_map_enum_field(&mut self, fld: MappedEnumField<'a, U>) -> Result<(), Self::Error> {
        todo!()
    }

    fn visit_map_message_field(&mut self, fld: MappedEmbedField<'a, U>) -> Result<(), Self::Error> {
        todo!()
    }

    fn visit_repeated_field(&mut self, fld: RepeatedField<'a, U>) -> Result<(), Self::Error> {
        visit_repeated_field(self, fld)
    }

    fn visit_repeated_scalar_field(
        &mut self,
        fld: RepeatedScalarField<'a, U>,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn visit_repeated_enum_field(
        &mut self,
        fld: RepeatedEnumField<'a, U>,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn visit_repeated_message_field(
        &mut self,
        fld: RepeatedEmbedField<'a, U>,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn visit_oneof_field(&mut self, fld: OneofField<'a, U>) -> Result<(), Self::Error> {
        visit_oneof_field(self, fld)
    }

    fn visit_real_oneof_field(&mut self, fld: RealOneofField<'a, U>) -> Result<(), Self::Error> {
        todo!()
    }

    fn visit_real_oneof_scalar_field(
        &mut self,
        fld: OneofScalarField<'a, U>,
    ) -> Result<(), Self::Error> {
        todo!()
    }
    fn visit_real_oneof_enum_field(
        &mut self,
        fld: OneofEnumField<'a, U>,
    ) -> Result<(), Self::Error> {
        todo!()
    }
    fn visit_real_oneof_message_field(
        &mut self,
        fld: OneofMessageField<'a, U>,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn visit_synthetic_oneof_field(
        &mut self,
        fld: SyntheticOneofField<'a, U>,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn visit_synthetic_oneof_scalar_field(
        &mut self,
        fld: OneofScalarField<'a, U>,
    ) -> Result<(), Self::Error> {
        todo!()
    }
    fn visit_synthetic_oneof_enum_field(
        &mut self,
        fld: OneofEnumField<'a, U>,
    ) -> Result<(), Self::Error> {
        todo!()
    }
    fn visit_synthetic_oneof_message_field(
        &mut self,
        fld: OneofMessageField<'a, U>,
    ) -> Result<(), Self::Error> {
        todo!()
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
    for ext in msg.defined_extensions() {
        v.visit_node(ext.into())?;
    }

    for enm in msg.enums() {
        v.visit_node(enm.into())?;
    }

    for nm in msg.messages() {
        v.visit_node(nm.into())?;
    }

    for fld in msg.fields() {
        v.visit_node(fld.into())?;
    }

    Ok(())
}

pub fn visit_service<'a, U, V>(v: &mut V, svc: Service<'a, U>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    for mth in svc.methods() {
        v.visit_node(mth.into())?;
    }
    Ok(())
}

pub fn visit_enum<'a, U, V>(v: &mut V, enm: Enum<'a, U>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    for ev in enm.values() {
        v.visit_node(ev.into())?;
    }

    Ok(())
}

pub fn visit_oneof<'a, U, V>(v: &mut V, one: Oneof<'a, U>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    todo!()
}

pub fn visit_field<'a, U, V>(v: &mut V, fld: Method<'a, U>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    todo!()
}

pub fn visit_map_field<'a, U, V>(v: &mut V, fld: MapField<'a, U>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    todo!()
}
pub fn visit_repeated_field<'a, U, V>(v: &mut V, f: RepeatedField<'a, U>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    todo!()
}

pub fn visit_oneof_field<'a, U, V>(v: &mut V, fld: OneofField<'a, U>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    todo!()
}

pub fn visit_real_oneof_field<'a, U, V>(v: &mut V, fld: OneofField<'a, U>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    todo!()
}

pub fn visit_synthetic_oneof_field<'a, U, V>(
    v: &mut V,
    fld: OneofField<'a, U>,
) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    todo!()
}

pub fn visit_extension<'a, U, V>(v: &mut V, ext: Extension<'a, U>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    todo!()
}
