use super::*;
use std::{marker::PhantomData, slice};

#[derive(Debug)]
pub struct EnumDescriptorIter<'a, U> {
    iter: slice::Iter<'a, prost_types::EnumDescriptorProto>,
    u: PhantomData<U>,
}
impl<'a, U> EnumDescriptorIter<'a, U> {
    pub fn len(&self) -> usize {
        self.iter.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
impl<'a, U> Iterator for EnumDescriptorIter<'a, U> {
    type Item = EnumDescriptor<'a, U>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|v| v.into())
    }
}
impl<'a, U> From<&'a Vec<prost_types::EnumDescriptorProto>> for EnumDescriptorIter<'a, U> {
    fn from(data: &'a Vec<prost_types::EnumDescriptorProto>) -> Self {
        EnumDescriptorIter {
            iter: data.iter(),
            u: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct EnumValueDescriptorIter<'a, U> {
    iter: std::slice::Iter<'a, prost_types::EnumValueDescriptorProto>,
    phantom: PhantomData<U>,
}

impl<'a, U> Iterator for EnumValueDescriptorIter<'a, U> {
    type Item = EnumValueDescriptor<'a, U>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(EnumValueDescriptor::from)
    }
}

impl<'a, U> From<&'a Vec<prost_types::EnumValueDescriptorProto>>
    for EnumValueDescriptorIter<'a, U>
{
    fn from(data: &'a Vec<prost_types::EnumValueDescriptorProto>) -> Self {
        EnumValueDescriptorIter {
            iter: data.iter(),
            phantom: PhantomData,
        }
    }
}

impl<'a, U> EnumValueDescriptorIter<'a, U> {
    pub fn len(&self) -> usize {
        self.iter.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug)]
pub struct FieldDescriptorIter<'a, U> {
    iter: std::slice::Iter<'a, prost_types::FieldDescriptorProto>,
    u: PhantomData<U>,
}

impl<'a, U> Iterator for FieldDescriptorIter<'a, U> {
    type Item = FieldDescriptor<'a, U>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|v| v.into())
    }
}
impl<'a, U> FieldDescriptorIter<'a, U> {
    pub fn len(&self) -> usize {
        self.iter.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
impl<'a, U> From<&'a Vec<prost_types::FieldDescriptorProto>> for FieldDescriptorIter<'a, U> {
    fn from(fields: &'a Vec<prost_types::FieldDescriptorProto>) -> Self {
        FieldDescriptorIter {
            iter: fields.iter(),
            u: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct MessageDescriptorIter<'a, U> {
    iter: std::slice::Iter<'a, prost_types::DescriptorProto>,
    phantom: PhantomData<U>,
}
impl<'a, U> MessageDescriptorIter<'a, U> {
    pub fn len(&self) -> usize {
        self.iter.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
impl<'a, U> Iterator for MessageDescriptorIter<'a, U> {
    type Item = MessageDescriptor<'a, U>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|v| v.into())
    }
}
impl<'a, U> From<&'a Vec<prost_types::DescriptorProto>> for MessageDescriptorIter<'a, U> {
    fn from(data: &'a Vec<prost_types::DescriptorProto>) -> Self {
        MessageDescriptorIter {
            iter: data.iter(),
            phantom: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct ServiceDescriptorIter<'a, U> {
    iter: std::slice::Iter<'a, prost_types::ServiceDescriptorProto>,
    phantom: PhantomData<U>,
}
impl<'a, U> Iterator for ServiceDescriptorIter<'a, U> {
    type Item = ServiceDescriptor<'a, U>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|d| d.into())
    }
}
impl<'a, U> From<&'a Vec<prost_types::ServiceDescriptorProto>> for ServiceDescriptorIter<'a, U> {
    fn from(data: &'a Vec<prost_types::ServiceDescriptorProto>) -> Self {
        ServiceDescriptorIter {
            iter: data.iter(),
            phantom: PhantomData,
        }
    }
}
impl<'a, U> ServiceDescriptorIter<'a, U> {
    pub fn len(&self) -> usize {
        self.iter.len()
    }
    pub fn is_empty(&self) -> bool {
        self.iter.len() == 0
    }
}
#[derive(Debug)]
pub struct MethodDescriptorIter<'a, U> {
    iter: std::slice::Iter<'a, prost_types::MethodDescriptorProto>,
    phantom: PhantomData<U>,
}
impl<'a, U> MethodDescriptorIter<'a, U> {
    pub fn len(&self) -> usize {
        self.iter.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
impl<'a, U> Iterator for MethodDescriptorIter<'a, U> {
    type Item = MethodDescriptor<'a, U>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|d| d.into())
    }
}
impl<'a, U> From<&'a Vec<prost_types::MethodDescriptorProto>> for MethodDescriptorIter<'a, U> {
    fn from(data: &'a Vec<prost_types::MethodDescriptorProto>) -> Self {
        MethodDescriptorIter {
            iter: data.iter(),
            phantom: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct OneofDescriptorIter<'a, U> {
    iter: std::slice::Iter<'a, prost_types::OneofDescriptorProto>,
    phantom: PhantomData<U>,
}
impl<'a, U> OneofDescriptorIter<'a, U> {
    pub fn len(&self) -> usize {
        self.iter.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
impl<'a, U> Iterator for OneofDescriptorIter<'a, U> {
    type Item = OneofDescriptor<'a, U>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|d| d.into())
    }
}
impl<'a, U> From<&'a Vec<prost_types::OneofDescriptorProto>> for OneofDescriptorIter<'a, U> {
    fn from(data: &'a Vec<prost_types::OneofDescriptorProto>) -> Self {
        OneofDescriptorIter {
            iter: data.iter(),
            phantom: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct EnumReservedRangeIter<'a, U> {
    iter: std::slice::Iter<'a, prost_types::enum_descriptor_proto::EnumReservedRange>,
    phantom: PhantomData<U>,
}
impl<'a, U> EnumReservedRangeIter<'a, U> {
    pub fn len(&self) -> usize {
        self.iter.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
impl<'a, U> Iterator for EnumReservedRangeIter<'a, U> {
    type Item = EnumReservedRange<'a, U>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|d| d.into())
    }
}

impl<'a, U> From<&'a [prost_types::enum_descriptor_proto::EnumReservedRange]>
    for EnumReservedRangeIter<'a, U>
{
    fn from(data: &'a [prost_types::enum_descriptor_proto::EnumReservedRange]) -> Self {
        EnumReservedRangeIter {
            iter: data.iter(),
            phantom: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct ExtensionRangeIter<'a, U> {
    iter: std::slice::Iter<'a, prost_types::descriptor_proto::ExtensionRange>,
    phantom: PhantomData<U>,
}
impl<'a, U> Iterator for ExtensionRangeIter<'a, U> {
    type Item = ExtensionRange<'a, U>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|d| d.into())
    }
}
impl<'a, U> From<&'a [prost_types::descriptor_proto::ExtensionRange]>
    for ExtensionRangeIter<'a, U>
{
    fn from(data: &'a [prost_types::descriptor_proto::ExtensionRange]) -> Self {
        ExtensionRangeIter {
            iter: data.iter(),
            phantom: PhantomData,
        }
    }
}
impl ExtensionRangeIter<'_, ()> {
    pub fn len(&self) -> usize {
        self.iter.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug)]
pub struct ReservedRangeIter<'a, U> {
    iter: std::slice::Iter<'a, prost_types::descriptor_proto::ReservedRange>,
    phantom: PhantomData<U>,
}
impl ReservedRangeIter<'_, ()> {
    pub fn len(&self) -> usize {
        self.iter.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
impl<'a, U> Iterator for ReservedRangeIter<'a, U> {
    type Item = ReservedRange<'a, U>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|d| d.into())
    }
}
impl<'a, U> From<&'a [prost_types::descriptor_proto::ReservedRange]> for ReservedRangeIter<'a, U> {
    fn from(data: &'a [prost_types::descriptor_proto::ReservedRange]) -> Self {
        ReservedRangeIter {
            iter: data.iter(),
            phantom: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct UninterpretedOptionsIter<'a, U> {
    iter: std::slice::Iter<'a, prost_types::UninterpretedOption>,
    u: PhantomData<U>,
}
impl<'a, U> Iterator for UninterpretedOptionsIter<'a, U> {
    type Item = UninterpretedOption<'a, U>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(UninterpretedOption::from)
    }
}

impl<'a, U> From<UninterpretedOptions<'a, U>> for UninterpretedOptionsIter<'a, U> {
    fn from(data: UninterpretedOptions<'a, U>) -> Self {
        UninterpretedOptionsIter {
            iter: data.opts.iter(),
            u: PhantomData,
        }
    }
}
impl<'a, U> From<&UninterpretedOptions<'a, U>> for UninterpretedOptionsIter<'a, U> {
    fn from(opts: &UninterpretedOptions<'a, U>) -> Self {
        UninterpretedOptionsIter {
            iter: opts.opts.iter(),
            u: PhantomData,
        }
    }
}
impl<'a, U> UninterpretedOptionsIter<'a, U> {
    pub fn len(&self) -> usize {
        self.iter.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug)]
pub struct LocationIter<'a, U> {
    iter: std::slice::Iter<'a, prost_types::source_code_info::Location>,
    phantom: PhantomData<U>,
}
impl<'a, U> LocationIter<'a, U> {
    pub fn len(&self) -> usize {
        self.iter.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
impl<'a, U> From<&SourceCodeInfo<'a, U>> for LocationIter<'a, U> {
    fn from(info: &SourceCodeInfo<'a, U>) -> Self {
        LocationIter {
            iter: info.info.location.iter(),
            phantom: PhantomData,
        }
    }
}
impl<'a, U> Iterator for LocationIter<'a, U> {
    type Item = Location<'a, U>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|l| l.into())
    }
}
impl<'a, U> Clone for LocationIter<'a, U> {
    fn clone(&self) -> Self {
        LocationIter {
            iter: self.iter.clone(),
            phantom: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct CommentsIter<'a, U> {
    iter: std::slice::Iter<'a, prost_types::source_code_info::Location>,
    phantom: PhantomData<U>,
}

impl<'a, U> CommentsIter<'a, U> {
    pub fn len(&self) -> usize {
        self.iter.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a, U> From<SourceCodeInfo<'a, U>> for CommentsIter<'a, U> {
    fn from(info: SourceCodeInfo<'a, U>) -> Self {
        CommentsIter {
            iter: info.info.location.iter(),
            phantom: PhantomData,
        }
    }
}
