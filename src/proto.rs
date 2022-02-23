use anyhow::bail;
use prost_types::field_descriptor_proto;

pub enum Syntax {
    Proto2,
    Proto3,
}

impl TryFrom<String> for Syntax {
    type Error = anyhow::Error;

    fn try_from(v: String) -> Result<Self, Self::Error> {
        match v.as_str() {
            "proto2" => Ok(Syntax::Proto2),
            "proto3" => Ok(Syntax::Proto3),
            "" => Ok(Syntax::Proto2),
            _ => bail!("invalid syntax: {}", v),
        }
    }
}

pub type Type = field_descriptor_proto::Type;


pub type Label = field_descriptor_proto::Label;