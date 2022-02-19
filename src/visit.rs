#![allow(unused_variables)]

use std::rc::Rc;

use crate::{Enum, EnumValue, Extension, Field, File, Message, Method, Oneof, Package, Service};

pub trait Visitor<U> {
    type Error;

    fn visit_package(&mut self, pkg: Rc<Package<U>>) -> Result<(), Self::Error> {
        visit_package(self, pkg)
    }

    fn visit_file(&mut self, file: Rc<File<U>>) -> Result<(), Self::Error> {
        todo!("not done")
    }

    fn visit_message(&mut self, msg: Rc<Message<U>>) -> Result<(), Self::Error> {
        todo!("not done")
    }

    fn visit_enum(&mut self, e: Rc<Enum<U>>) -> Result<(), Self::Error> {
        todo!("not done")
    }

    fn visit_enum_value(&mut self, value: Rc<EnumValue<U>>) -> Result<(), Self::Error> {
        todo!("not done")
    }

    fn visit_field(&mut self, file: Rc<Field<U>>) -> Result<(), Self::Error> {
        todo!("not done")
    }

    fn visit_extension(&mut self, ext: Rc<Extension<U>>) -> Result<(), Self::Error> {
        todo!("not done")
    }

    fn visit_one_of(&mut self, one_of: Rc<Oneof<U>>) -> Result<(), Self::Error> {
        todo!("not done")
    }

    fn visit_service(&mut self, service: Rc<Service<U>>) -> Result<(), Self::Error> {
        todo!("not done")
    }

    fn visit_method(&mut self, method: Rc<Method<U>>) -> Result<(), Self::Error> {
        todo!("not done")
    }
}

pub fn visit_package<U, V>(v: &mut V, pkg: Rc<Package<U>>) -> Result<(), V::Error>
where
    V: Visitor<U> + ?Sized,
{
    for file in pkg.files() {
        v.visit_file(file)?;
    }
    Ok(())
}

pub fn visit_file<U, V>(v: &mut V, file: Rc<File<U>>) -> Result<(), V::Error>
where
    V: Visitor<U> + ?Sized,
{
    for msg in file.messages.iter().cloned() {
        v.visit_message(msg)?;
    }
    for e in file.enums.iter().cloned() {
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
