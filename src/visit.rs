#![allow(unused_variables)]

use std::rc::Rc;

use crate::{
    Enum, EnumValue, Extension, Field, File, MapEnumField, MapField, MapMessageField,
    MapScalarField, Message, MessageField, Method, Oneof, OneofEnumField, OneofField,
    OneofMessageField, OneofScalarField, Package, RepeatedEnumField, RepeatedField,
    RepeatedMessageField, RepeatedScalarField, ScalarField, Service,
};

pub trait Accept<'a, U, V: Visitor<'a, U>> {
    fn accept(&self, visitor: &mut V) -> Result<(), V::Error>;
}

pub fn walk<'a, U, V: Visitor<'a, U>, A: Accept<'a, U, V>>(
    v: &mut V,
    node: &A,
) -> Result<(), V::Error> {
    node.accept(v)
}

pub trait Visitor<'a, U>: Sized {
    type Error;
    fn done(&mut self) -> bool {
        false
    }
    fn visit_package(&mut self, pkg: Rc<Package<'a, U>>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_file(&mut self, f: Rc<File<'a, U>>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_message(&mut self, msg: Rc<Message<'a, U>>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_enum(&mut self, enm: Rc<Enum<'a, U>>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_enum_value(&mut self, val: Rc<EnumValue<'a, U>>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_field(&mut self, fld: Field<'a, U>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_scalar_field(&mut self, fld: Rc<ScalarField<'a, U>>) -> Result<(), Self::Error> {
        Ok(())
    }
    fn visit_message_field(&mut self, fld: Rc<MessageField<'a, U>>) -> Result<(), Self::Error> {
        Ok(())
    }
    fn visit_map_field(&mut self, fld: MapField<'a, U>) -> Result<(), Self::Error> {
        Ok(())
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
        Ok(())
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
        Ok(())
    }

    fn visit_oneof_scalar(&mut self, fld: Rc<OneofScalarField<'a, U>>) -> Result<(), Self::Error> {
        Ok(())
    }
    fn visit_oneof_enum(&mut self, fld: Rc<OneofEnumField<'a, U>>) -> Result<(), Self::Error> {
        Ok(())
    }
    fn visit_oneof_message(
        &mut self,
        fld: Rc<OneofMessageField<'a, U>>,
    ) -> Result<(), Self::Error> {
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
