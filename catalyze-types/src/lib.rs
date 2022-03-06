mod descriptor;
mod enums;
mod path;
mod source_code_info;
mod syntax;
mod uninterpreted_option;
mod well_known;

pub use descriptor::*;
pub use enums::*;
pub use path::*;
pub use path::*;
pub use source_code_info::*;
pub use syntax::*;
pub use uninterpreted_option::*;
pub use well_known::*;

pub trait Impl<'a> {
    // Descriptors

    type FileDescriptor: FileDescriptor<'a, Self>;
    type EnumDescriptor: EnumDescriptor<'a, Self>;
    type EnumValueDescriptor: EnumValueDescriptor<'a, Self>;
    type ServiceDescriptor: ServiceDescriptor<'a, Self>;
    type MethodDescriptor: MethodDescriptor<'a, Self>;
    type FieldDescriptor: FieldDescriptor<'a, Self>;
    type OneofDescriptor: OneofDescriptor<'a, Self>;
    type MessageDescriptor: MessageDescriptor<'a, Self>;

    // Descriptor Options

    type FieldOptions: FieldOptions<'a, Self>;
    type OneofOptions: OneofOptions<'a, Self>;
    type MessageOptions: MessageOptions<'a, Self>;
    type EnumOptions: EnumOptions<'a, Self>;
    type EnumValueOptions: EnumValueOptions<'a, Self>;
    type ServiceOptions: ServiceOptions<'a, Self>;
    type MethodOptions: MethodOptions<'a, Self>;
    type FileOptions: FileOptions<'a, Self>;

    // Descriptor Options Supporting Types

    type NamePart: NamePart<'a, Self>;
    type SourceCodeInfo: SourceCodeInfo<'a, Self>;
    type Location: Location<'a, Self>;
    type ReservedRanges: ReservedRanges<'a, Self>;
    type ReservedRange: ReservedRange<'a, Self>;
    type EnumReservedRanges: EnumReservedRanges<'a, Self>;
    type EnumReservedRange: EnumReservedRange<'a, Self>;

    // Iterators

    type FileDescriptorIter: ExactSizeIterator<Item = Self::FileDescriptor>;
    type EnumDescriptorIter: ExactSizeIterator<Item = Self::EnumDescriptor>;
    type EnumValueDescriptorIter: ExactSizeIterator<Item = Self::EnumValueDescriptor>;
    type ServiceDescriptorIter: ExactSizeIterator<Item = Self::ServiceDescriptor>;
    type MethodDescriptorIter: ExactSizeIterator<Item = Self::MethodDescriptorIter>;
    type FieldDescriptorIter: ExactSizeIterator<Item = Self::FieldDescriptor>;
    type OneofDescriptorIter: ExactSizeIterator<Item = Self::OneofDescriptor>;
    type MessageDescriptorIter: ExactSizeIterator<Item = Self::MessageDescriptor>;

    type SourceCodeInfoIter: ExactSizeIterator<Item = Self::SourceCodeInfo>;
    type LocationIter: ExactSizeIterator<Item = Self::Location>;
    type NamePartIter: ExactSizeIterator<Item = Self::NamePart>;
    type UninterpretedOptionIter: ExactSizeIterator<Item = Self::UninterpretedOption>;
    type ReservedRangeIter: ExactSizeIterator<Item = Self::ReservedRange>;
    type EnumReservedRangeIter: ExactSizeIterator<Item = Self::EnumReservedRange>;
}
