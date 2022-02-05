use crate::name::{Name, ToScreamingKebabCase};
use anyhow::Result;

pub use heck::{
    ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase,
    ToTitleCase, ToUpperCamelCase,
};
use std::{fmt::Display, string::ToString};

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

pub trait Language: Sized + Clone {
    fn is_keyword<T: ToString>(&self, s: T) -> Result<Keyword>;
    fn to_screaming_kebab_case(&self, name: &Name<Self>) -> Name<Self> {
        Name {
            val: name.val.to_shouty_kebab_case(),
            lang: self.clone(),
        }
    }
    fn to_screaming_snake_case(&self, name: &Name<Self>) -> Name<Self> {
        Name {
            val: name.val.to_shouty_kebab_case(),
            lang: self.clone(),
        }
    }
    fn to_kebab_case(&self, name: &Name<Self>) -> Name<Self> {
        Name {
            val: name.val.to_kebab_case(),
            lang: self.clone(),
        }
    }
    fn to_camel_case(&self, name: &Name<Self>) -> Name<Self> {
        Name {
            val: name.val.to_kebab_case(),
            lang: self.clone(),
        }
    }
    fn to_pascal_case(&self, name: &Name<Self>) -> Name<Self> {
        Name {
            val: name.val.to_kebab_case(),
            lang: self.clone(),
        }
    }
    fn to_snake_case(&self, name: &Name<Self>) -> Name<Self> {
        Name {
            val: name.val.to_kebab_case(),
            lang: self.clone(),
        }
    }
}
