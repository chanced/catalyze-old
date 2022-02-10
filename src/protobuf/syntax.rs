use anyhow::bail;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// The syntax in which a protocol buffer element is defined.
pub enum Syntax {
    Proto2 = 0,
    Proto3 = 1,
}

impl From<i32> for Syntax {
    fn from(i: i32) -> Self {
        match i {
            0 => Syntax::Proto2,
            1 => Syntax::Proto3,
            _ => Syntax::Proto2,
        }
    }
}

impl TryFrom<&str> for Syntax {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim().to_lowercase().as_str() {
            "proto2" => Ok(Syntax::Proto2),
            "proto3" => Ok(Syntax::Proto3),
            x => bail!("unknown syntax: {}", x),
        }
    }
}
