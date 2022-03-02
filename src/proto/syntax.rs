use anyhow::bail;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Syntax {
    Proto2,
    Proto3,
}

impl Syntax {
    pub fn supports_required_prefix(&self) -> bool {
        match self {
            Syntax::Proto2 => true,
            Syntax::Proto3 => false,
        }
    }
    pub fn is_proto2(&self) -> bool {
        match self {
            Syntax::Proto2 => true,
            Syntax::Proto3 => false,
        }
    }
    pub fn is_proto3(&self) -> bool {
        match self {
            Syntax::Proto2 => false,
            Syntax::Proto3 => true,
        }
    }
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

impl ToString for Syntax {
    fn to_string(&self) -> String {
        match self {
            Syntax::Proto2 => "proto2",
            Syntax::Proto3 => "proto3",
        }
        .to_string()
    }
}

impl From<prost_types::Syntax> for Syntax {
    fn from(syntax: prost_types::Syntax) -> Self {
        match syntax {
            prost_types::Syntax::Proto2 => Syntax::Proto2,
            prost_types::Syntax::Proto3 => Syntax::Proto3,
        }
    }
}
