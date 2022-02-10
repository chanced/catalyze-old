use super::{Opt, ReservedRange, UninterpretedOption};

/// Range of reserved numeric values. Reserved values may not be used by
/// entries in the same enum. Reserved ranges may not overlap.
///
/// Note that this is distinct from DescriptorProto.ReservedRange in that it
/// is inclusive such that it can appropriately represent the entire int32
/// domain.
#[derive(Clone, PartialEq, Debug)]
pub struct EnumReservedRange {
    /// Inclusive.
    pub start: Option<i32>,
    /// Inclusive.
    pub end: Option<i32>,
}

impl EnumReservedRange {
    pub(crate) fn new(desc: &prost_types::enum_descriptor_proto::EnumReservedRange) -> Self {
        EnumReservedRange {
            start: desc.start,
            end: desc.end,
        }
    }
}

/// Describes an enum type.
#[derive(Clone, PartialEq, Debug)]
pub struct EnumDescriptor {
    pub name: Option<String>,
    pub values: Vec<EnumValueDescriptor>,
    pub options: Option<EnumOptions>,
    /// Range of reserved numeric values. Reserved numeric values may not be used
    /// by enum values in the same enum declaration. Reserved ranges may not
    /// overlap.
    pub reserved_ranges: Vec<EnumReservedRange>,
    /// Reserved enum value names, which may not be reused. A given name may only
    /// be reserved once.
    pub reserved_names: Vec<String>,
}

impl EnumDescriptor {
    pub(crate) fn new(ed: &prost_types::EnumDescriptorProto) -> Self {
        EnumDescriptor {
            name: ed.name.clone(),
            values: ed
                .value
                .iter()
                .map(|v| EnumValueDescriptor::new(v))
                .collect(),
            options: ed.options.map(|o| EnumOptions::new(o)),
            reserved_ranges: ed
                .reserved_range
                .iter()
                .map(|r| EnumReservedRange::new(r))
                .collect(),
            reserved_names: ed.reserved_name.iter().map(|r| r.clone()).collect(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct EnumOptions {
    pub allow_alias: Option<bool>,
    /// Is this enum deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the enum, or it will be completely ignored; in the very least, this
    /// is a formalization for deprecating enums.
    pub deprecated: Option<bool>,
    pub uninterpreted_options: Vec<UninterpretedOption>,
}

impl EnumOptions {
    pub(crate) fn new(eo: prost_types::EnumOptions) -> Self {
        EnumOptions {
            allow_alias: eo.allow_alias.clone(),
            deprecated: eo.deprecated.clone(),
            uninterpreted_options: eo
                .uninterpreted_option
                .iter()
                .map(|uo| UninterpretedOption::new(uo))
                .collect(),
        }
    }
}

/// Describes a value within an enum.
#[derive(Clone, PartialEq, Debug)]
pub struct EnumValueDescriptor {
    pub name: Option<String>,
    pub number: Option<i32>,
    pub options: Option<EnumValueOptions>,
}
impl EnumValueDescriptor {
    pub(crate) fn new(evd: &prost_types::EnumValueDescriptorProto) -> Self {
        EnumValueDescriptor {
            name: evd.name.clone(),
            number: evd.number.clone(),
            options: evd.options.map(|o| EnumValueOptions::new(&o)),
        }
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct EnumValueOptions {
    /// Is this enum value deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the enum value, or it will be completely ignored; in the very least,
    /// this is a formalization for deprecating enum values.
    pub deprecated: Option<bool>,
    /// The parser stores options it doesn't recognize here. See above.
    pub uninterpreted_options: Vec<UninterpretedOption>,
}

impl EnumValueOptions {
    pub(crate) fn new(evo: &prost_types::EnumValueOptions) -> Self {
        EnumValueOptions {
            deprecated: evo.deprecated.clone(),
            uninterpreted_options: evo
                .uninterpreted_option
                .iter()
                .map(|uo| UninterpretedOption::new(uo))
                .collect(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Enum {
    pub name: String,
    pub values: Vec<EnumValue>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct EnumValue {
    pub name: String,
    pub number: i32,
    /// Protocol buffer options.
    pub options: Vec<Opt>,
}
