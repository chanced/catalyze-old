use crate::name::Name;
// use anyhow::Result;

use heck::AsLowerCamelCase;
pub use heck::{
    ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase,
    ToTitleCase, ToUpperCamelCase,
};
use std::string::ToString;

pub trait IsKeyword {
    fn is_keyword(&self, s: &str) -> Option<Vec<KeywordCase>>;
}

pub enum KeywordCase {
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
}

pub trait ToScreamingSnakeCase: ToOwned {
    fn to_screaming_snake_case(&self) -> Self::Owned;
}

pub trait ToScreamingKebabCase: ToOwned {
    fn to_screaming_kebab_case(&self) -> Self::Owned;
}

impl<T: ToShoutySnakeCase + ?Sized> ToScreamingSnakeCase for T {
    fn to_screaming_snake_case(&self) -> Self::Owned {
        self.to_shouty_snake_case()
    }
}
impl<T: ToShoutyKebabCase + ?Sized> ToScreamingKebabCase for T {
    fn to_screaming_kebab_case(&self) -> Self::Owned {
        self.to_shouty_kebab_case()
    }
}

impl<T: ToLowerCamelCase> ToCamelCase for T {
    fn to_camel_case(&self) -> Self::Owned {
        self.to_lower_camel_case()
    }
}

pub trait ToCamelCase: ToOwned {
    fn to_camel_case(&self) -> Self::Owned;
}
impl ToCamelCase for str {
    fn to_camel_case(&self) -> String {
        AsLowerCamelCase(self).to_string()
    }
}

pub trait ToCase: Sized + Clone {
    fn to_screaming_kebab_case(&self, name: &Name<Self>) -> Name<Self> {
        name.with_value(&name.as_str().to_screaming_kebab_case())
    }
    fn to_screaming_snake_case(&self, name: &Name<Self>) -> Name<Self> {
        name.with_value(&name.as_str().to_screaming_snake_case())
    }
    fn to_kebab_case(&self, name: &Name<Self>) -> Name<Self> {
        name.with_value(&name.as_str().to_kebab_case())
    }
    fn to_camel_case(&self, name: &Name<Self>) -> Name<Self> {
        name.with_value(&name.as_str().to_lower_camel_case())
    }
    fn to_pascal_case(&self, name: &Name<Self>) -> Name<Self> {
        name.with_value(&name.as_str().to_pascal_case())
    }
    fn to_snake_case(&self, name: &Name<Self>) -> Name<Self> {
        name.with_value(&name.as_str().to_snake_case())
    }
}
