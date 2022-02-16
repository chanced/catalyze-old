use crate::name::{Name, ToScreamingKebabCase};
// use anyhow::Result;

pub use heck::{
    ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase,
    ToTitleCase, ToUpperCamelCase,
};
use std::string::ToString;

use super::Unspecified;

pub enum Keyword {
    /// SCREAMING-KEBAB-CASE
    ScreamingKebab,
    /// SCREAMING_SNAKE_CASE, also referred to as SHOUTY_SNAKE_CASE
    ScreamingSnake,
    /// PascalCase, also referred to as UpperCamelCase
    Pascal,
    /// camelCase, also referred to as lowerCamelCase
    Camel,
    /// snake_case
    Snake,
    /// Not a keyword for the given language
    None,
}

pub trait Lang: Sized + Clone {
    type Error;
    fn name() -> &'static str;

    fn is_keyword<T: ToString>(&self, s: T) -> Result<Keyword, Self::Error>;
    fn to_screaming_kebab_case(&self, name: &Name<Self>) -> Name<Self> {
        Name::new(&name.to_string().to_shouty_kebab_case(), self.clone())
    }
    fn to_screaming_snake_case(&self, name: &Name<Self>) -> Name<Self> {
        Name::new(&name.to_string().to_shouty_snake_case(), self.clone())
    }
    fn to_kebab_case(&self, name: &Name<Self>) -> Name<Self> {
        Name::new(&name.to_string().to_kebab_case(), self.clone())
    }
    fn to_camel_case(&self, name: &Name<Self>) -> Name<Self> {
        Name::new(&name.to_string().to_lower_camel_case(), self.clone())
    }
    fn to_pascal_case(&self, name: &Name<Self>) -> Name<Self> {
        Name::new(&name.to_string().to_pascal_case(), self.clone())
    }
    fn to_snake_case(&self, name: &Name<Self>) -> Name<Self> {
        Name::new(&name.to_string().to_snake_case(), self.clone())
    }
}
