use super::*;
use std::slice;

#[derive(Debug)]
pub struct EnumDescriptorIter<'a> {
    iter: slice::Iter<'a, prost_types::EnumDescriptorProto>,
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
impl<'a> From<&'a Vec<prost_types::EnumDescriptorProto>> for EnumDescriptorIter<'a> {
    fn from(data: &'a Vec<prost_types::EnumDescriptorProto>) -> Self {
        EnumDescriptorIter { iter: data.iter() }
    }
}

#[derive(Debug)]
pub struct FileDescriptorIter<'a> {
    iter: std::slice::Iter<'a, prost_types::FileDescriptorProto>,
}

impl<'a> Iterator for FileDescriptorIter<'a> {
    type Item = FileDescriptor<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(FileDescriptor::from)
    }
}

impl<'a> From<&'a Vec<prost_types::FileDescriptorProto>> for FileDescriptorIter<'a> {
    fn from(data: &'a Vec<prost_types::FileDescriptorProto>) -> Self {
        FileDescriptorIter { iter: data.iter() }
    }
}

impl<'a> FileDescriptorIter<'a> {
    pub fn len(&self) -> usize {
        self.iter.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug)]
pub struct EnumValueDescriptorIter<'a> {
    iter: std::slice::Iter<'a, prost_types::EnumValueDescriptorProto>,
}

impl<'a> Iterator for EnumValueDescriptorIter<'a> {
    type Item = EnumValueDescriptor<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(EnumValueDescriptor::from)
    }
}

impl<'a> From<&'a Vec<prost_types::EnumValueDescriptorProto>> for EnumValueDescriptorIter<'a> {
    fn from(data: &'a Vec<prost_types::EnumValueDescriptorProto>) -> Self {
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

#[derive(Debug)]
pub struct FieldDescriptorIter<'a> {
    iter: std::slice::Iter<'a, prost_types::FieldDescriptorProto>,
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
impl<'a> From<&'a Vec<prost_types::FieldDescriptorProto>> for FieldDescriptorIter<'a> {
    fn from(fields: &'a Vec<prost_types::FieldDescriptorProto>) -> Self {
        FieldDescriptorIter {
            iter: fields.iter(),
        }
    }
}

#[derive(Debug)]
pub struct MessageDescriptorIter<'a> {
    iter: std::slice::Iter<'a, prost_types::DescriptorProto>,
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
impl<'a> From<&'a Vec<prost_types::DescriptorProto>> for MessageDescriptorIter<'a> {
    fn from(data: &'a Vec<prost_types::DescriptorProto>) -> Self {
        MessageDescriptorIter { iter: data.iter() }
    }
}

#[derive(Debug)]
pub struct ServiceDescriptorIter<'a> {
    iter: std::slice::Iter<'a, prost_types::ServiceDescriptorProto>,
}
impl<'a> Iterator for ServiceDescriptorIter<'a> {
    type Item = ServiceDescriptor<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|d| d.into())
    }
}
impl<'a> From<&'a Vec<prost_types::ServiceDescriptorProto>> for ServiceDescriptorIter<'a> {
    fn from(data: &'a Vec<prost_types::ServiceDescriptorProto>) -> Self {
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

#[derive(Debug)]
pub struct MethodDescriptorIter<'a> {
    iter: std::slice::Iter<'a, prost_types::MethodDescriptorProto>,
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
impl<'a> From<&'a Vec<prost_types::MethodDescriptorProto>> for MethodDescriptorIter<'a> {
    fn from(data: &'a Vec<prost_types::MethodDescriptorProto>) -> Self {
        MethodDescriptorIter { iter: data.iter() }
    }
}

#[derive(Debug)]
pub struct OneofDescriptorIter<'a> {
    iter: std::slice::Iter<'a, prost_types::OneofDescriptorProto>,
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
impl<'a> From<&'a Vec<prost_types::OneofDescriptorProto>> for OneofDescriptorIter<'a> {
    fn from(data: &'a Vec<prost_types::OneofDescriptorProto>) -> Self {
        OneofDescriptorIter { iter: data.iter() }
    }
}

#[derive(Debug)]
pub struct EnumReservedRangeIter<'a> {
    iter: std::slice::Iter<'a, prost_types::enum_descriptor_proto::EnumReservedRange>,
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
impl<'a> From<&'a [prost_types::enum_descriptor_proto::EnumReservedRange]>
    for EnumReservedRangeIter<'a>
{
    fn from(data: &'a [prost_types::enum_descriptor_proto::EnumReservedRange]) -> Self {
        EnumReservedRangeIter { iter: data.iter() }
    }
}

#[derive(Debug)]
pub struct ExtensionRangeIter<'a> {
    iter: std::slice::Iter<'a, prost_types::descriptor_proto::ExtensionRange>,
}
impl<'a> Iterator for ExtensionRangeIter<'a> {
    type Item = ExtensionRange<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|d| d.into())
    }
}
impl<'a> From<&'a [prost_types::descriptor_proto::ExtensionRange]> for ExtensionRangeIter<'a> {
    fn from(data: &'a [prost_types::descriptor_proto::ExtensionRange]) -> Self {
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

#[derive(Debug)]
pub struct ReservedRangeIter<'a> {
    iter: std::slice::Iter<'a, prost_types::descriptor_proto::ReservedRange>,
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
impl<'a> From<&'a [prost_types::descriptor_proto::ReservedRange]> for ReservedRangeIter<'a> {
    fn from(data: &'a [prost_types::descriptor_proto::ReservedRange]) -> Self {
        ReservedRangeIter { iter: data.iter() }
    }
}

#[derive(Debug)]
pub struct UninterpretedOptionsIter<'a> {
    iter: std::slice::Iter<'a, prost_types::UninterpretedOption>,
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

#[derive(Debug)]
pub struct LocationIter<'a> {
    iter: std::slice::Iter<'a, prost_types::source_code_info::Location>,
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
impl<'a> Clone for LocationIter<'a> {
    fn clone(&self) -> Self {
        LocationIter {
            iter: self.iter.clone(),
        }
    }
}
