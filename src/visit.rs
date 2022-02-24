#![allow(unused_variables)]

use std::rc::Rc;

use crate::{
    AsNode, Enum, EnumValue, Extension, Field, File, MapEnumField, MapField, MapMessageField,
    MapScalarField, Message, MessageField, Method, Node, Oneof, OneofEnumField, OneofField,
    OneofMessageField, OneofScalarField, Package, RepeatedEnumField, RepeatedField,
    RepeatedMessageField, RepeatedScalarField, ScalarField, Service, WellKnownTypeField,
    WktMessageField,
};

pub trait Visitor<'a, U>: Sized {
    type Error;

    fn visit_node(&mut self, node: Node<'a, U>) -> Result<(), Self::Error> {
        visit_node(self, node)
    }

    fn visit_package(&mut self, pkg: Rc<Package<'a, U>>) -> Result<(), Self::Error> {
        visit_package(self, pkg)
    }

    fn visit_file(&mut self, f: Rc<File<'a, U>>) -> Result<(), Self::Error> {
        visit_file(self, f)
    }

    fn visit_message(&mut self, msg: Rc<Message<'a, U>>) -> Result<(), Self::Error> {
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
        visit_field(self, fld)
    }

    fn visit_scalar_field(&mut self, fld: Rc<ScalarField<'a, U>>) -> Result<(), Self::Error> {
        Ok(())
    }
    fn visit_message_field(&mut self, fld: Rc<MessageField<'a, U>>) -> Result<(), Self::Error> {
        Ok(())
    }
    fn visit_map_field(&mut self, fld: MapField<'a, U>) -> Result<(), Self::Error> {
        visit_map_field(self, fld)
    }

    fn visit_map_scalar_field(
        &mut self,
        fld: Rc<MapScalarField<'a, U>>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_map_enum_field(&mut self, fld: Rc<MapEnumField<'a, U>>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_map_message_field(
        &mut self,
        fld: Rc<MapMessageField<'a, U>>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_repeated_field(&mut self, fld: RepeatedField<'a, U>) -> Result<(), Self::Error> {
        visit_repeated_field(self, fld)
    }

    fn visit_repeated_scalar_field(
        &mut self,
        fld: Rc<RepeatedScalarField<'a, U>>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_repeated_enum_field(
        &mut self,
        fld: Rc<RepeatedEnumField<'a, U>>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_repeated_message_field(
        &mut self,
        fld: Rc<RepeatedMessageField<'a, U>>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_oneof_field(&mut self, fld: OneofField<'a, U>) -> Result<(), Self::Error> {
        visit_oneof_field(self, fld)
    }

    fn visit_oneof_scalar_field(
        &mut self,
        fld: Rc<OneofScalarField<'a, U>>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
    fn visit_oneof_enum_field(
        &mut self,
        fld: Rc<OneofEnumField<'a, U>>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
    fn visit_oneof_message_field(
        &mut self,
        fld: Rc<OneofMessageField<'a, U>>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
    fn visit_oneof_message(
        &mut self,
        fld: Rc<OneofMessageField<'a, U>>,
    ) -> Result<(), Self::Error> {
        Ok()
    }

    fn visit_wkt_field(&mut self, fld: WellKnownTypeField<'a, U>) -> Result<(), Self::Error> {
        visit_field(self, fld)
    }

    fn visit_wkt_message_field(&mut self, fld: WktMessageField<'a, U>) -> Result<(), Self::Error> {}

    fn visit_wkt_enum_field(&mut self, fld: WktMessageField<'a, U>) -> Result<(), Self::Error> {
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
    }
}

pub fn visit_package<'a, U, V>(v: &mut V, pkg: Rc<Package<'a, U>>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    for file in pkg.files() {
        file.accept(v)?;
    }
    Ok(())
}

pub fn visit_file<'a, U, V>(v: &mut V, f: Rc<File<'a, U>>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    for msg in f.messages() {
        msg.accept(v)?;
    }
    for enm in f.enums() {
        enm.accept(v)?;
    }
    for svc in f.services() {
        svc.accept(v)?;
    }
    for ext in f.defined_extensions() {
        ext.accept(v)?;
    }
    Ok(())
}

pub fn visit_message<'a, U, V>(v: &mut V, msg: Rc<Message<'a, U>>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    for ext in msg.defined_extensions() {
        v.visit_node(ext)?;
    }

    for enm in msg.enums() {
        v.visit_node(enm)?;
    }

    for nm in msg.messages() {
        v.visit_node(nm)?;
    }

    for fld in msg.fields() {
        v.visit_node(fld)
    }

    Ok(())
}

pub fn visit_service<'a, U, V>(v: &mut V, svc: Rc<Service<'a, U>>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    for mth in svc.methods() {
        v.visit_node(mth.as_node())?;
    }
    Ok(())
}

pub fn visit_enum<'a, U, V>(v: &mut V, enm: Rc<Enum<'a, U>>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    for ev in enm.values() {
        v.visit_node(ev.as_node())?;
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

pub fn visit_map_field<'a, U, V>(v: &mut V, fld: Rc<MapField<'a, U>>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    todo!()
}
pub fn visit_repeated_field<'a, U, V>(
    v: &mut V,
    f: Rc<RepeatedField<'a, U>>,
) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    todo!()
}

pub fn visit_oneof_field<'a, U, V>(v: &mut V, fld: Rc<OneofField<'a, U>>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    todo!()
}

pub fn visit_extension<'a, U, V>(v: &mut V, ext: Rc<Extension<'a, U>>) -> Result<(), V::Error>
where
    V: Visitor<'a, U>,
{
    todo!()
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn test_visit_package() {
    //     let pkg = Package::new(, lang)
    //         name: crate::Name::new("foo".to_string(), lang::NotSpecified),
    //         files: RefCell::new(vec![]),
    //     };
    //     let mut v = MockVisitor {};
    //     visit_package(&mut v, &pkg).unwrap();
    // }
}
