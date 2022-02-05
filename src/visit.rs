use std::rc::Rc;

use super::node::*;
use crate::File;
use crate::Language;
use crate::Message;
use crate::Package;
pub trait Visitor<L: Language> {
    type Error;

    fn visit_package(&mut self, pkg: Package<L>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_file(&mut self, file: File<L>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_message(&mut self, msg: Rc<Message<L>>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_enum(&mut self, e: Enum) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_enum_value(&mut self, ev: EnumValue) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_field(&mut self, f: Field) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_extension(&mut self, extension: Extension) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_one_of(&mut self, one_of: OneOf) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_service(&mut self, service: Service) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_method(&mut self, method: Method) -> Result<(), Self::Error> {
        Ok(())
    }
}

// pub fn visit_package<'a, V, E>(&mut V, pkg: &'a Package) -> Result<(), E>
// where V: Visitor<'a> + ?Sized
// {
//     Ok(())
// }

// pub fn visit_file(&mut self, file: &'a File) -> Result<(), Self::Error> {
//     Ok(())
// }

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
