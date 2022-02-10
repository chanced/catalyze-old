// use std::str::FromStr;

pub const WELL_KNNOWN_TYPE_PACKAGE: &str = "google.protobuf";
use anyhow::bail;
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WellKnownType {
    Any,
    Duration,
    Empty,
    Struct,
    Timestamp,
    Value,
    List,
    Double,
    Float,
    Int64,
    Uint64,
    Int32,
    Uint32,
    Bool,
    String,
    Bytes,
}
impl std::str::FromStr for WellKnownType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> ::std::result::Result<WellKnownType, Self::Err> {
        match s {
            "Any" => Ok(WellKnownType::Any),
            "Duration" => Ok(WellKnownType::Duration),
            "Empty" => Ok(WellKnownType::Empty),
            "Struct" => Ok(WellKnownType::Struct),
            "Timestamp" => Ok(WellKnownType::Timestamp),
            "Value" => Ok(WellKnownType::Value),
            "ListValue" => Ok(WellKnownType::List),
            "DoubleValue" => Ok(WellKnownType::Double),
            "FloatValue" => Ok(WellKnownType::Float),
            "Int64Value" => Ok(WellKnownType::Int64),
            "UInt64Value" => Ok(WellKnownType::Uint64),
            "Int32Value" => Ok(WellKnownType::Int32),
            "UInt32Value" => Ok(WellKnownType::Uint32),
            "BoolValue" => Ok(WellKnownType::Bool),
            "StringValue" => Ok(WellKnownType::String),
            "BytesValue" => Ok(WellKnownType::Bytes),
            _ => bail!("Unknown WellKnownType"),
        }
    }
}
