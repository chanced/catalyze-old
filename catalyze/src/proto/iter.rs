use super::*;
use std::slice;

#[derive(Debug, Clone)]
pub struct EnumDescriptorIter<'a> {
    iter: slice::Iter<'a, protobuf::descriptor::EnumDescriptorProto>,
}
impl<'a> EnumDescriptorIter<'a> {
    pub fn len(&self) -> usize {
        self.iter.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
impl<'a> Iterator for EnumDescriptorIter<'a> {
    type Item = EnumDescriptor<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|v| v.into())
    }
}
impl<'a> From<&'a Vec<protobuf::descriptor::EnumDescriptorProto>> for EnumDescriptorIter<'a> {
    fn from(data: &'a Vec<protobuf::descriptor::EnumDescriptorProto>) -> Self {
        EnumDescriptorIter { iter: data.iter() }
    }
}

#[derive(Debug, Clone)]
pub struct FileDescriptorIter<'a> {
    iter: std::slice::Iter<'a, protobuf::descriptor::FileDescriptorProto>,
}

impl<'a> FileDescriptorIter<'a> {
    pub fn len(&self) -> usize {
        self.iter.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a> Iterator for FileDescriptorIter<'a> {
    type Item = FileDescriptor<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(FileDescriptor::from)
    }
}

impl<'a> From<&'a Vec<protobuf::descriptor::FileDescriptorProto>> for FileDescriptorIter<'a> {
    fn from(data: &'a Vec<protobuf::descriptor::FileDescriptorProto>) -> Self {
        FileDescriptorIter { iter: data.iter() }
    }
}

#[derive(Debug, Clone)]
pub struct EnumValueDescriptorIter<'a> {
    iter: std::slice::Iter<'a, protobuf::descriptor::EnumValueDescriptorProto>,
}

impl<'a> Iterator for EnumValueDescriptorIter<'a> {
    type Item = EnumValueDescriptor<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(EnumValueDescriptor::from)
    }
}

impl<'a> From<&'a Vec<protobuf::descriptor::EnumValueDescriptorProto>>
    for EnumValueDescriptorIter<'a>
{
    fn from(data: &'a Vec<protobuf::descriptor::EnumValueDescriptorProto>) -> Self {
        EnumValueDescriptorIter { iter: data.iter() }
    }
}

impl<'a> EnumValueDescriptorIter<'a> {
    pub fn len(&self) -> usize {
        self.iter.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug, Clone)]
pub struct FieldDescriptorIter<'a> {
    iter: std::slice::Iter<'a, protobuf::descriptor::FieldDescriptorProto>,
}

impl<'a> Iterator for FieldDescriptorIter<'a> {
    type Item = FieldDescriptor<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|v| v.into())
    }
}
impl<'a> FieldDescriptorIter<'a> {
    pub fn len(&self) -> usize {
        self.iter.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
impl<'a> From<&'a Vec<protobuf::descriptor::FieldDescriptorProto>> for FieldDescriptorIter<'a> {
    fn from(fields: &'a Vec<protobuf::descriptor::FieldDescriptorProto>) -> Self {
        FieldDescriptorIter {
            iter: fields.iter(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MessageDescriptorIter<'a> {
    iter: std::slice::Iter<'a, protobuf::descriptor::DescriptorProto>,
}
impl<'a> MessageDescriptorIter<'a> {
    pub fn len(&self) -> usize {
        self.iter.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
impl<'a> Iterator for MessageDescriptorIter<'a> {
    type Item = MessageDescriptor<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|v| v.into())
    }
}
impl<'a> From<&'a Vec<protobuf::descriptor::DescriptorProto>> for MessageDescriptorIter<'a> {
    fn from(data: &'a Vec<protobuf::descriptor::DescriptorProto>) -> Self {
        MessageDescriptorIter { iter: data.iter() }
    }
}

#[derive(Debug, Clone)]
pub struct ServiceDescriptorIter<'a> {
    iter: std::slice::Iter<'a, protobuf::descriptor::ServiceDescriptorProto>,
}
impl<'a> Iterator for ServiceDescriptorIter<'a> {
    type Item = ServiceDescriptor<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|d| d.into())
    }
}
impl<'a> From<&'a Vec<protobuf::descriptor::ServiceDescriptorProto>> for ServiceDescriptorIter<'a> {
    fn from(data: &'a Vec<protobuf::descriptor::ServiceDescriptorProto>) -> Self {
        ServiceDescriptorIter { iter: data.iter() }
    }
}
impl<'a> ServiceDescriptorIter<'a> {
    pub fn len(&self) -> usize {
        self.iter.len()
    }
    pub fn is_empty(&self) -> bool {
        self.iter.len() == 0
    }
}

#[derive(Debug, Clone)]
pub struct MethodDescriptorIter<'a> {
    iter: std::slice::Iter<'a, protobuf::descriptor::MethodDescriptorProto>,
}
impl<'a> MethodDescriptorIter<'a> {
    pub fn len(&self) -> usize {
        self.iter.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
impl<'a> Iterator for MethodDescriptorIter<'a> {
    type Item = MethodDescriptor<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|d| d.into())
    }
}
impl<'a> From<&'a Vec<protobuf::descriptor::MethodDescriptorProto>> for MethodDescriptorIter<'a> {
    fn from(data: &'a Vec<protobuf::descriptor::MethodDescriptorProto>) -> Self {
        MethodDescriptorIter { iter: data.iter() }
    }
}

#[derive(Debug, Clone)]
pub struct OneofDescriptorIter<'a> {
    iter: std::slice::Iter<'a, protobuf::descriptor::OneofDescriptorProto>,
}
impl<'a> OneofDescriptorIter<'a> {
    pub fn len(&self) -> usize {
        self.iter.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
impl<'a> Iterator for OneofDescriptorIter<'a> {
    type Item = OneofDescriptor<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|d| d.into())
    }
}
impl<'a> From<&'a Vec<protobuf::descriptor::OneofDescriptorProto>> for OneofDescriptorIter<'a> {
    fn from(data: &'a Vec<protobuf::descriptor::OneofDescriptorProto>) -> Self {
        OneofDescriptorIter { iter: data.iter() }
    }
}

#[derive(Debug, Clone)]
pub struct EnumReservedRangeIter<'a> {
    iter: std::slice::Iter<'a, protobuf::descriptor::enum_descriptor_proto::EnumReservedRange>,
}
impl<'a> EnumReservedRangeIter<'a> {
    pub fn len(&self) -> usize {
        self.iter.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
impl<'a> Iterator for EnumReservedRangeIter<'a> {
    type Item = EnumReservedRange<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|d| d.into())
    }
}
impl<'a> From<&'a [protobuf::descriptor::enum_descriptor_proto::EnumReservedRange]>
    for EnumReservedRangeIter<'a>
{
    fn from(data: &'a [protobuf::descriptor::enum_descriptor_proto::EnumReservedRange]) -> Self {
        EnumReservedRangeIter { iter: data.iter() }
    }
}

#[derive(Debug, Clone)]
pub struct ExtensionRangeIter<'a> {
    iter: std::slice::Iter<'a, protobuf::descriptor::descriptor_proto::ExtensionRange>,
}
impl<'a> Iterator for ExtensionRangeIter<'a> {
    type Item = ExtensionRange<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|d| d.into())
    }
}
impl<'a> From<&'a [protobuf::descriptor::descriptor_proto::ExtensionRange]>
    for ExtensionRangeIter<'a>
{
    fn from(data: &'a [protobuf::descriptor::descriptor_proto::ExtensionRange]) -> Self {
        ExtensionRangeIter { iter: data.iter() }
    }
}
impl<'a> ExtensionRangeIter<'a> {
    pub fn len(&self) -> usize {
        self.iter.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug, Clone)]
pub struct ReservedRangeIter<'a> {
    iter: std::slice::Iter<'a, protobuf::descriptor::descriptor_proto::ReservedRange>,
}
impl<'a> ReservedRangeIter<'a> {
    pub fn len(&self) -> usize {
        self.iter.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
impl<'a> Iterator for ReservedRangeIter<'a> {
    type Item = ReservedRange<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|d| d.into())
    }
}
impl<'a> From<&'a [protobuf::descriptor::descriptor_proto::ReservedRange]>
    for ReservedRangeIter<'a>
{
    fn from(data: &'a [protobuf::descriptor::descriptor_proto::ReservedRange]) -> Self {
        ReservedRangeIter { iter: data.iter() }
    }
}

#[derive(Debug, Clone)]
pub struct UninterpretedOptionsIter<'a> {
    iter: std::slice::Iter<'a, protobuf::descriptor::UninterpretedOption>,
}
impl<'a> Iterator for UninterpretedOptionsIter<'a> {
    type Item = UninterpretedOption<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(UninterpretedOption::from)
    }
}
impl<'a> From<UninterpretedOptions<'a>> for UninterpretedOptionsIter<'a> {
    fn from(data: UninterpretedOptions<'a>) -> Self {
        UninterpretedOptionsIter {
            iter: data.opts.iter(),
        }
    }
}
impl<'a> From<&UninterpretedOptions<'a>> for UninterpretedOptionsIter<'a> {
    fn from(opts: &UninterpretedOptions<'a>) -> Self {
        UninterpretedOptionsIter {
            iter: opts.opts.iter(),
        }
    }
}
impl<'a> UninterpretedOptionsIter<'a> {
    pub fn len(&self) -> usize {
        self.iter.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug, Clone)]
pub struct LocationIter<'a> {
    iter: std::slice::Iter<'a, protobuf::descriptor::source_code_info::Location>,
}
impl<'a> LocationIter<'a> {
    pub fn len(&self) -> usize {
        self.iter.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
impl<'a> From<&SourceCodeInfo<'a>> for LocationIter<'a> {
    fn from(info: &SourceCodeInfo<'a>) -> Self {
        LocationIter {
            iter: info.info.location.iter(),
        }
    }
}
impl<'a> Iterator for LocationIter<'a> {
    type Item = Location<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|l| l.into())
    }
}
