#![allow(unused_variables)]

use std::rc::Rc;

use crate::{
    Enum, EnumField, EnumValue, Extension, Field, File, IntoNode, MapEnumField, MapField,
    MapMessageField, MapScalarField, Message, MessageField, Method, Node, Oneof, OneofEnumField,
    OneofField, OneofMessageField, OneofScalarField, Package, RepeatedEnumField, RepeatedField,
    RepeatedMessageField, RepeatedScalarField, ScalarField, Service, WellKnownTypeField,
    WktMessageField,
};

pub trait Visitor<'a, U>: Sized {
    type Error;

    fn visit_node(&mut self, node: Node<'a, U>) -> Result<(), Self::Error> {
        visit_node(self, node)
    }

    fn visit_package(&mut self, pkg: Package<'a, U>) -> Result<(), Self::Error> {
        visit_package(self, pkg)
    }

    fn visit_file(&mut self, f: Rc<File<'a, U>>) -> Result<(), Self::Error> {
        visit_file(self, f)
    }

    fn visit_message(&mut self, msg: Message<'a, U>) -> Result<(), Self::Error> {
        visit_message(self, msg)
    }

    fn visit_enum(&mut self, enm: Rc<Enum<'a, U>>) -> Result<(), Self::Error> {
        visit_enum(self, enm)
    }

    fn visit_enum_value(&mut self, val: Rc<EnumValue<'a, U>>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_extension(&mut self, ext: Rc<Extension<'a, U>>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_oneof(&mut self, oneof: Rc<Oneof<'a, U>>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_service(&mut self, svc: Rc<Service<'a, U>>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_method(&mut self, mtd: Rc<Method<'a, U>>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_field(&mut self, fld: Field<'a, U>) -> Result<(), Self::Error> {
        visit_node(self, fld.into_node())
    }

    fn visit_scalar_field(&mut self, fld: ScalarField<'a, U>) -> Result<(), Self::Error> {
        Ok(())
    }
    fn visit_message_field(&mut self, fld: MessageField<'a, U>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_enum_field(&mut self, fld: EnumField<'a, U>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_map_field(&mut self, fld: MapField<'a, U>) -> Result<(), Self::Error> {
        visit_map_field(self, fld)
    }

    fn visit_map_scalar_field(&mut self, fld: MapScalarField<'a, U>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_map_enum_field(&mut self, fld: MapEnumField<'a, U>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_map_message_field(&mut self, fld: MapMessageField<'a, U>) -> Result<(), Self::Error> {
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

    fn visit_repeated_message_field(
        &mut self,
        fld: RepeatedMessageField<'a, U>,
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
    fn visit_oneof_enum_field(&mut self, fld: OneofEnumField<'a, U>) -> Result<(), Self::Error> {
        Ok(())
    }
    fn visit_oneof_message_field(
        &mut self,
        fld: OneofMessageField<'a, U>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
    fn visit_oneof_message(&mut self, fld: OneofMessageField<'a, U>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_wkt_field(&mut self, fld: WellKnownTypeField<'a, U>) -> Result<(), Self::Error> {
        visit_wkt_field(self, fld)
    }

    fn visit_well_known_message_field(
        &mut self,
        fld: WktMessageField<'a, U>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_well_known_enum_field(
        &mut self,
        fld: WktMessageField<'a, U>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub fn visit_node<'a, U, V>(v: &mut V, node: Node<'a, U>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    match node {
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
        v.visit_node(file.into_node())?;
    }
    Ok(())
}

pub fn visit_file<'a, U, V>(v: &mut V, f: Rc<File<'a, U>>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    for msg in f.messages() {
        v.visit_node(msg.into_node())?;
    }
    for enm in f.enums() {
        v.visit_node(enm.into_node())?;
    }
    for svc in f.services() {
        v.visit_node(svc.into_node())?;
    }
    for ext in f.defined_extensions() {
        v.visit_node(ext.into_node())?;
    }
    Ok(())
}

pub fn visit_message<'a, U, V>(v: &mut V, msg: Message<'a, U>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    for ext in msg.defined_extensions() {
        v.visit_node(ext.into_node())?;
    }

    for enm in msg.enums() {
        v.visit_node(enm.into_node())?;
    }

    for nm in msg.messages() {
        v.visit_node(nm.into_node())?;
    }

    for fld in msg.fields() {
        v.visit_node(fld.into_node())?;
    }

    Ok(())
}

pub fn visit_service<'a, U, V>(v: &mut V, svc: Rc<Service<'a, U>>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    for mth in svc.methods() {
        v.visit_node(mth.into_node())?;
    }
    Ok(())
}

pub fn visit_enum<'a, U, V>(v: &mut V, enm: Rc<Enum<'a, U>>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    for ev in enm.values() {
        v.visit_node(ev.into_node())?;
    }

    Ok(())
}

pub fn visit_oneof<'a, U, V>(v: &mut V, one: Rc<Oneof<'a, U>>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    todo!()
}

pub fn visit_field<'a, U, V>(v: &mut V, fld: Rc<Method<'a, U>>) -> Result<(), V::Error>
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
    match fld.as_ref() {
        OneofField::Scalar(s) => v.visit_oneof_scalar_field(s.clone()),
        OneofField::Enum(e) => v.visit_oneof_enum_field(e.clone()),
        OneofField::Message(m) => v.visit_oneof_message_field(m.clone()),
    }
}

pub fn visit_wkt_field<'a, U, V>(v: &mut V, fld: WellKnownTypeField<'a, U>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    match fld.as_ref() {
        WellKnownTypeField::Any(f)
        | WellKnownTypeField::Api(f)
        | WellKnownTypeField::BoolValue(f)
        | WellKnownTypeField::BytesValue(f)
        | WellKnownTypeField::DoubleValue(f)
        | WellKnownTypeField::Duration(f)
        | WellKnownTypeField::Empty(f)
        | WellKnownTypeField::Enum(f)
        | WellKnownTypeField::EnumValue(f)
        | WellKnownTypeField::Field(f)
        | WellKnownTypeField::FieldMask(f)
        | WellKnownTypeField::FloatValue(f)
        | WellKnownTypeField::Int32Value(f)
        | WellKnownTypeField::Int64Value(f)
        | WellKnownTypeField::ListValue(f)
        | WellKnownTypeField::Method(f)
        | WellKnownTypeField::Mixin(f)
        | WellKnownTypeField::Option(f)
        | WellKnownTypeField::SourceContext(f)
        | WellKnownTypeField::StringValue(f)
        | WellKnownTypeField::Struct(f)
        | WellKnownTypeField::Timestamp(f)
        | WellKnownTypeField::Type(f)
        | WellKnownTypeField::UInt32Value(f)
        | WellKnownTypeField::UInt64Value(f)
        | WellKnownTypeField::Value(f) => v.visit_message_field(f.message_field()),

        WellKnownTypeField::FieldCardinality(f)
        | WellKnownTypeField::FieldKind(f)
        | WellKnownTypeField::NullValue(f)
        | WellKnownTypeField::Syntax(f) => v.visit_enum_field(f.enum_field()),
    }
}
pub fn visit_extension<'a, U, V>(v: &mut V, ext: Rc<Extension<'a, U>>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    todo!()
}
