use std::borrow::Borrow;
use std::rc::Rc;

use crate::lang::{self, Lang};
use crate::{Enum, EnumValue, Extension, Field, File, Message, Method, Oneof, Package, Service};

pub trait Visitor<L: Lang> {
    type Error;

    fn visit_package(&mut self, pkg: Rc<Package<L>>) -> Result<(), Self::Error> {
        visit_package(self, pkg)
    }

    fn visit_file(&mut self, file: Rc<File<L>>) -> Result<(), Self::Error> {
        todo!("not done")
    }

    fn visit_message(&mut self, msg: Rc<Message<L>>) -> Result<(), Self::Error> {
        todo!("not done")
    }

    fn visit_enum(&mut self, e: Rc<Enum<L>>) -> Result<(), Self::Error> {
        todo!("not done")
    }

    fn visit_enum_value(&mut self, ev: Rc<EnumValue<L>>) -> Result<(), Self::Error> {
        todo!("not done")
    }

    fn visit_field(&mut self, file: Rc<Field<L>>) -> Result<(), Self::Error> {
        todo!("not done")
    }

    fn visit_extension(&mut self, extension: Rc<Extension<L>>) -> Result<(), Self::Error> {
        todo!("not done")
    }

    fn visit_one_of(&mut self, one_of: Rc<Oneof<L>>) -> Result<(), Self::Error> {
        todo!("not done")
    }

    fn visit_service(&mut self, service: Rc<Service<L>>) -> Result<(), Self::Error> {
        todo!("not done")
    }

    fn visit_method(&mut self, method: Rc<Method<L>>) -> Result<(), Self::Error> {
        todo!("not done")
    }
}

pub fn visit_package<L, V>(v: &mut V, pkg: Rc<Package<L>>) -> Result<(), V::Error>
where
    V: Visitor<L> + ?Sized,
    L: Lang,
{
    for file in pkg.files() {
        v.visit_file(file)?;
    }
    Ok(())
}

pub fn visit_file<L, V>(v: &mut V, file: Rc<File<L>>) -> Result<(), V::Error>
where
    V: Visitor<L> + ?Sized,
    L: Lang,
{
    for msg in file.messages() {
        v.visit_message(msg)?;
    }
    for e in file.enums() {
        v.visit_enum(e)?;
    }

    Ok(())
}

// pub fn visit_message(&mut self, file: &'a File) -> Result<(), Self::Error> {
//     Ok(())
// }

// pub fn visit_enum(&mut self, enum_: &'a Enum) -> Result<(), Self::Error> {
//     Ok(())
// }

// pub fn visit_enum_value(&mut self, enum_value: EnumValue) -> Result<(), Self::Error> {
//     Ok(())
// }

// pub fn visit_field(&mut self, field: Field) -> Result<(), Self::Error> {
//     Ok(())
// }

// pub fn visit_extension(&mut self, extension: Extension) -> Result<(), Self::Error> {
//     Ok(())
// }

// pub fn visit_one_of(&mut self, one_of: OneOf) -> Result<(), Self::Error> {
//     Ok(())
// }

// pub fn visit_service(&mut self, service: Service) -> Result<(), Self::Error> {
//     Ok(())
// }

// pub fn visit_method(&mut self, method: Method) -> Result<(), Self::Error> {
//     Ok(())
// }
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockVisitor<'a, L: Lang> {
        pkg: &'a Package<L>,
    }
    impl<'a, L: Lang> Visitor<L> for MockVisitor<'a, L> {
        type Error = anyhow::Error;
    }

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
