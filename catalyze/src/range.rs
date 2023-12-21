/// Range of reserved tag numbers. Reserved tag numbers may not be used by
/// fields or extension ranges in the same message. Reserved ranges may
/// not overlap.
#[derive(Debug, Clone, Copy)]
pub struct Reserved {
    start: Option<i32>,
    end: Option<i32>,
}
impl Reserved {
    /// Inclusive.
    pub fn start(&self) -> i32 {
        self.range.start
    }

    /// Exclusive.
    pub fn end(&self) -> i32 {
        self.range.end
    }

    pub fn in_range(&self, val: i32) -> bool {
        self.start() <= val && val < self.end()
    }
}
impl PartialEq for Reserved {
    fn eq(&self, other: &Self) -> bool {
        self.start() == other.start() && self.end() == other.end()
    }
}
impl From<protobuf::descriptor::descriptor_proto::ReservedRange> for Reserved {
    fn from(range: protobuf::descriptor::descriptor_proto::ReservedRange) -> Self {
        Reserved {
            start: range.start,
            end: range.end,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Extension {
    // message fields
    // @@protoc_insertion_point(field:google.protobuf.DescriptorProto.ExtensionRange.start)
    pub start: ::std::option::Option<i32>,
    // @@protoc_insertion_point(field:google.protobuf.DescriptorProto.ExtensionRange.end)
    pub end: ::std::option::Option<i32>,
    // @@protoc_insertion_point(field:google.protobuf.DescriptorProto.ExtensionRange.options)
    pub options: crate::MessageField<protobuf::descriptor::ExtensionRangeOptions>,
    // special fields
    // @@protoc_insertion_point(special_field:google.protobuf.DescriptorProto.ExtensionRange.special_fields)
}
impl PartialEq for Extension {
    fn eq(&self, other: &Self) -> bool {
        self.range.start() == other.start() && self.end() == other.end()
    }
}
impl Extension {
    /// Inclusive.
    pub fn start(&self) -> i32 {
        self.range.start()
    }
    /// Exclusive.
    pub fn end(&self) -> i32 {
        self.range.end()
    }
    pub fn in_range(&self, val: i32) -> bool {
        self.start() <= val && val < self.end()
    }
}
impl From<&'a protobuf::descriptor::descriptor_proto::ExtensionRange> for Extension {
    fn from(range: &'a protobuf::descriptor::descriptor_proto::ExtensionRange) -> Self {
        Extension { range }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ExtensionRanges {
    ranges: &'a [protobuf::descriptor::descriptor_proto::ExtensionRange],
}
impl ExtensionRanges {
    pub fn iter(&self) -> ExtensionRangeIter {
        self.ranges.into()
    }
    pub fn len(&self) -> usize {
        self.ranges.len()
    }
    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }
    pub fn first(&self) -> Option<Extension> {
        self.ranges.first().map(|r| r.into())
    }
    pub fn last(&self) -> Option<Extension> {
        self.ranges.last().map(|r| r.into())
    }
    pub fn get(&self, n: usize) -> Option<Extension> {
        self.ranges.get(n).map(|r| r.into())
    }
}
impl IntoIterator for ExtensionRanges {
    type Item = Extension;
    type IntoIter = ExtensionRangeIter;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl From<&'a Vec<protobuf::descriptor::descriptor_proto::ExtensionRange>> for ExtensionRanges {
    fn from(ranges: &'a Vec<protobuf::descriptor::descriptor_proto::ExtensionRange>) -> Self {
        ExtensionRanges { ranges }
    }
}

// Range of reserved numeric values. Reserved values may not be used by
/// entries in the same enum. Reserved ranges may not overlap.
///
/// Note that this is distinct from DescriptorProto.ReservedRange in that it
/// is inclusive such that it can appropriately represent the entire int32
/// domain.
#[derive(Debug, PartialEq)]
pub struct EnumReservedRange {
    rr: &'a protobuf::descriptor::enum_descriptor_proto::EnumReservedRange,
}
impl From<&'a protobuf::descriptor::enum_descriptor_proto::EnumReservedRange>
    for EnumReservedRange
{
    fn from(r: &'a protobuf::descriptor::enum_descriptor_proto::EnumReservedRange) -> Self {
        Self { rr: r }
    }
}
impl EnumReservedRange {
    /// Inclusive
    pub fn start(&self) -> i32 {
        self.rr.start()
    }
    /// Inclusive
    pub fn end(&self) -> i32 {
        self.rr.end()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EnumReservedRanges {
    ranges: &'a [protobuf::descriptor::enum_descriptor_proto::EnumReservedRange],
}
impl IntoIterator for EnumReservedRanges {
    type Item = EnumReservedRange;
    type IntoIter = EnumReservedRangeIter;
    fn into_iter(self) -> Self::IntoIter {
        self.ranges.into()
    }
}
impl EnumReservedRanges {
    pub fn iter(&self) -> EnumReservedRangeIter {
        self.ranges.into()
    }
    pub fn len(&self) -> usize {
        self.ranges.len()
    }
    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }
    pub fn get(&self, index: usize) -> Option<EnumReservedRange> {
        self.ranges.get(index).map(|r| r.into())
    }
    pub fn is_range_reserved(&self, min: i32, max: i32) -> bool {
        self.iter().any(|r| r.start() <= min && r.end() >= max)
    }
    pub fn is_in_reserved_range(&self, num: i32) -> bool {
        self.iter().any(|r| r.start() <= num && r.end() >= num)
    }
}
impl From<&'a Vec<protobuf::descriptor::enum_descriptor_proto::EnumReservedRange>>
    for EnumReservedRanges
{
    fn from(
        ranges: &'a Vec<protobuf::descriptor::enum_descriptor_proto::EnumReservedRange>,
    ) -> Self {
        Self { ranges }
    }
}
