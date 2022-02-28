use crate::util::{Generic, ToCase};
use crate::WELL_KNNOWN_TYPE_PACKAGE;
pub use heck::{
    AsLowerCamelCase, ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutyKebabCase,
    ToShoutySnakeCase, ToSnakeCase, ToTitleCase, ToUpperCamelCase,
};
use std::cell::RefCell;
use std::fmt::Write;
use std::hash::{Hash, Hasher};
use std::ops;
use std::rc::Rc;

use std::str::FromStr;
use std::{fmt, ops::Add};

pub trait Named<U> {
    fn name(&self) -> Name<U>;
}

pub struct Name<U> {
    val: String,
    pub util: Rc<RefCell<U>>,
}

impl<'a, U> Clone for Name<U> {
    fn clone(&self) -> Self {
        Self {
            val: self.val.clone(),
            util: self.util.clone(),
        }
    }
}

impl<'a, U> Hash for Name<U> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.val.hash(state);
    }
}

impl<'a, U> Name<U> {
    pub fn new(val: &str, util: Rc<RefCell<U>>) -> Self {
        Self {
            val: val.to_owned(),
            util,
        }
    }
    /// Assign returns a new `Name` with the contents of `val` and a cloned copy
    /// of `Rc<RefCell<U>>`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let n = Name::new("foo", Rc::new(RefCell::new(util::Generic{})));
    /// let n2 = n.assign("bar");
    /// assert_eq!(n2, "bar");
    /// ```
    pub fn with_value(&self, val: &str) -> Self {
        Self {
            val: val.to_owned(),
            util: self.util.clone(),
        }
    }
    /// Assign returns a new `Name` with the contents of `val` and a cloned copy
    /// of `Rc<RefCell<U>>`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let n = Name::new("foo", Rc::new(RefCell::new(util::Generic{})));
    /// let n2 = n.assign(String::from("bar"));
    /// assert_eq!(n2, "bar");
    /// ```
    pub fn assign_string(&self, val: String) -> Self {
        Self {
            val,
            util: self.util.clone(),
        }
    }
    // Returns a byte slice of this `Name`'s value.
    pub fn as_bytes(&self) -> &[u8] {
        &self.val.as_bytes()
    }
    pub fn util(&self) -> Rc<RefCell<U>> {
        self.util.clone()
    }
}

impl<'a, U> PartialEq for Name<U> {
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&self.val, &other.val)
    }
    fn ne(&self, other: &Self) -> bool {
        PartialEq::ne(&self.val, &other.val)
    }
}
impl<'a, U> PartialEq<String> for Name<U> {
    fn eq(&self, other: &String) -> bool {
        PartialEq::eq(&self.val, other)
    }
    fn ne(&self, other: &String) -> bool {
        PartialEq::ne(&self.val, other)
    }
}
impl<'a, U> PartialEq<str> for Name<U> {
    fn eq(&self, other: &str) -> bool {
        PartialEq::eq(self.as_str(), other)
    }
    fn ne(&self, other: &str) -> bool {
        PartialEq::ne(self.as_str(), other)
    }
}

impl Default for Name<Generic> {
    fn default() -> Self {
        Self {
            val: Default::default(),
            util: Rc::new(RefCell::new(Generic)),
        }
    }
}
impl FromStr for Name<Generic> {
    type Err = core::convert::Infallible;
    #[inline]
    fn from_str(s: &str) -> Result<Name<Generic>, Self::Err> {
        Ok(Name::from(s))
    }
}

impl From<&str> for Name<Generic> {
    #[inline]
    fn from(s: &str) -> Self {
        Self {
            val: String::from(s),
            util: Rc::new(RefCell::new(Generic)),
        }
    }
}

impl<'a, U> Write for Name<U> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.val.write_str(s)
    }
}

impl<'a, U> ops::Deref for Name<U> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl<'a, U> Name<U> {
    pub(crate) fn is_well_known_package(&self) -> bool {
        self.val.starts_with(WELL_KNNOWN_TYPE_PACKAGE)
    }
    pub fn as_str(&self) -> &str {
        self.val.as_str()
    }
}

impl<'a, U> Add<Self> for Name<U> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            val: format!("{}{}", self.val, other.val),
            util: self.util,
        }
    }
}
impl<'a, U> Add<&str> for Name<U> {
    type Output = Self;
    fn add(self, rhs: &str) -> Self::Output {
        Name::new(&(self.val + rhs), self.util)
    }
}

impl<'a, U> Add<String> for Name<U> {
    type Output = Self;
    fn add(self, rhs: String) -> Self::Output {
        Name::new(&(self.val + rhs.as_str()), self.util)
    }
}

pub trait ToScreamingSnakeCase: ToOwned {
    fn to_screaming_snake_case(&self) -> Self::Owned;
}

pub trait ToScreamingKebabCase: ToOwned {
    fn to_screaming_kebab_case(&self) -> Self::Owned;
}

impl<T: ToShoutySnakeCase> ToScreamingSnakeCase for T {
    fn to_screaming_snake_case(&self) -> Self::Owned {
        self.to_shouty_snake_case()
    }
}
impl<T: ToShoutyKebabCase> ToScreamingKebabCase for T {
    fn to_screaming_kebab_case(&self) -> Self::Owned {
        self.to_shouty_kebab_case()
    }
}

impl<T: ToLowerCamelCase> ToCamelCase for T {
    fn to_camel_case(&self) -> Self::Owned {
        self.to_lower_camel_case()
    }
}
impl ToCamelCase for str {
    fn to_camel_case(&self) -> String {
        AsLowerCamelCase(self).to_string()
    }
}

impl<U: ToCase> Name<U> {
    pub fn to_camel_case(&self) -> Self {
        self.util().borrow_mut().to_camel_case(self)
    }
    pub fn to_snake_case(&self) -> Self {
        self.util().borrow_mut().to_snake_case(self)
    }
    pub fn to_kebab_case(&self) -> Self {
        self.util().borrow_mut().to_kebab_case(self)
    }
    pub fn to_screaming_snake_case(&self) -> Self {
        self.util().borrow_mut().to_screaming_snake_case(self)
    }
    pub fn to_screaming_kebab_case(&self) -> Self {
        self.util().borrow_mut().to_screaming_kebab_case(self)
    }
    pub fn to_pascal_case(&self) -> Self {
        self.util().borrow_mut().to_pascal_case(self)
    }
}

pub trait ToCamelCase: ToOwned {
    fn to_camel_case(&self) -> Self::Owned;
}

impl<U: ToCase> ToKebabCase for Name<U> {
    fn to_kebab_case(&self) -> Self {
        self.util.borrow().to_kebab_case(self)
    }
}
impl<U: ToCase> ToSnakeCase for Name<U> {
    fn to_snake_case(&self) -> Self {
        self.util.borrow().to_snake_case(self)
    }
}

impl<U: ToCase> ToPascalCase for Name<U> {
    fn to_pascal_case(&self) -> Self {
        self.util.borrow().to_pascal_case(self)
    }
}

impl<U: ToCase> ToScreamingSnakeCase for Name<U> {
    fn to_screaming_snake_case(&self) -> Self {
        self.util.borrow().to_screaming_snake_case(self)
    }
}

impl<'a, U> fmt::Debug for Name<U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl<'a, U> fmt::Display for Name<U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

// impl<U: Language> ToKebabCase for Name<U> {
//     fn to_kebab_case(&self) -> Name<U> {
//         self.lang.to_kebab_case(self.clone())
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::Rust;
    #[test]
    fn test_to_kebab() {
        let n = Name::new("hello_world", Rc::new(RefCell::new(Rust {})));
        assert_eq!(n.to_kebab_case().to_string(), "hello-world".to_string());
    }
    #[test]
    fn test_to_pascal() {
        let n = Name::new("hello_world", Rc::new(RefCell::new(Rust {})));
        assert_eq!(n.to_pascal_case().to_string(), "HelloWorld".to_string());
    }
}
