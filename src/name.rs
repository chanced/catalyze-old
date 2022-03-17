use heck::{
    ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase,
};

use std::fmt::Write;
use std::hash::{Hash, Hasher};
use std::ops;

use std::{fmt, ops::Add};

pub struct Name {
    val: String,
}

impl Clone for Name {
    fn clone(&self) -> Self {
        Self {
            val: self.val.clone(),
        }
    }
}

impl Hash for Name {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.val.hash(state);
    }
}
impl Eq for Name {}
impl Name {
    pub fn new(val: &str) -> Self {
        Self {
            val: val.to_owned(),
        }
    }
    /// Assign returns a new `Name` with the contents of `val` and a cloned copy
    /// of `Rc<U>`.
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
        }
    }
    /// Assign returns a new `Name` with the contents of `val` and a cloned copy
    /// of `Rc<U>`.
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
        Self { val }
    }
    // Returns a byte slice of this `Name`'s value.
    pub fn as_bytes(&self) -> &[u8] {
        self.val.as_bytes()
    }
}

impl Write for Name {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.val.write_str(s)
    }
}

impl ops::Deref for Name {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl Name {
    pub fn as_str(&self) -> &str {
        self.val.as_str()
    }
}

impl From<&str> for Name {
    fn from(val: &str) -> Self {
        Self { val: val.to_string() }
    }
}

impl Add<Self> for Name {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            val: format!("{}{}", self.val, other.val),
        }
    }
}
impl Add<&str> for Name {
    type Output = Self;
    fn add(self, rhs: &str) -> Self::Output {
        (self.val + rhs).into()
    }
}

impl Add<&String> for Name {
    type Output = Self;
    fn add(self, rhs: &String) -> Self::Output {
        self.add(rhs.as_str())
    }
}

impl Add<String> for Name {
    type Output = Self;
    fn add(self, rhs: String) -> Self::Output {
        self.add(rhs.as_str())
    }
}

impl PartialEq<str> for Name {
    fn eq(&self, other: &str) -> bool {
        PartialEq::eq(&self.val, other)
    }
}

impl PartialEq<Name> for str {
    fn eq(&self, other: &Name) -> bool {
        PartialEq::eq(self, &other.val)
    }
}

impl PartialEq<Name> for String {
    fn eq(&self, other: &Name) -> bool {
        PartialEq::eq(&self.as_str(), &other.val)
    }
}
impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&self.val, &other.val)
    }
}
impl PartialEq<String> for Name {
    fn eq(&self, other: &String) -> bool {
        PartialEq::eq(&self.val, other)
    }
}

impl Name {
    pub fn to_camel_case(&self) -> Self {
        self.val.to_lower_camel_case().into()
    }
    pub fn to_kebab_case(&self) -> Self {
        self.val.to_kebab_case().into()
    }
    pub fn to_screaming_kebab_case(&self) -> Self {
        self.val.to_shouty_kebab_case().into()
    }
    pub fn to_snake_case(&self) -> Self {
        self.val.to_snake_case().into()
    }
    pub fn to_screaming_snake_case(&self) -> Self {
        self.val.to_shouty_snake_case().into()
    }
    pub fn to_pascal_case(&self) -> Self {
        self.val.to_pascal_case().into()
    }
}

impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl From<String> for Name {
    fn from(val: String) -> Self {
        Name::new(&val)
    }
}

impl Default for Name {
    fn default() -> Self {
        Name::new("")
    }
}

// impl<U: Language> ToKebabCase for Name {
//     fn to_kebab_case(&self) -> Name {
//         self.lang.to_kebab_case(self.clone())
//     }
// }

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_to_kebab() {
        let n = Name::new("hello_world");
        assert_eq!(n.to_kebab_case().to_string(), "hello-world".to_string());
    }
    #[test]
    fn test_to_pascal() {
        let n = Name::new("hello_world");
        assert_eq!(n.to_pascal_case(), "HelloWorld".to_string());
    }
}
