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

    type FileDescriptor: FileDescriptor<'a, Impl = Self>;
    type EnumDescriptor: EnumDescriptor<'a, Impl = Self>;
    type EnumValueDescriptor: EnumValueDescriptor<'a, Impl = Self>;
    type ServiceDescriptor: ServiceDescriptor<'a, Impl = Self>;
    type MethodDescriptor: MethodDescriptor<'a, Impl = Self>;
    type FieldDescriptor: FieldDescriptor<'a, Impl = Self>;
    type OneofDescriptor: OneofDescriptor<'a, Impl = Self>;
    type MessageDescriptor: MessageDescriptor<'a, Impl = Self>;

    // Descriptor Options

    type FieldOptions: FieldOptions<'a, Impl = Self>;
    type OneofOptions: OneofOptions<'a, Impl = Self>;
    type MessageOptions: MessageOptions<'a, Impl = Self>;
    type EnumOptions: EnumOptions<'a, Impl = Self>;
    type EnumValueOptions: EnumValueOptions<'a, Impl = Self>;
    type ServiceOptions: ServiceOptions<'a, Impl = Self>;
    type MethodOptions: MethodOptions<'a, Impl = Self>;
    type FileOptions: FileOptions<'a, Impl = Self>;

    // Descriptor Options Supporting Types

    type NamePart: NamePart<'a, Impl = Self>;
    type SourceCodeInfo: SourceCodeInfo<'a, Impl = Self>;
    type UninterpretedOption: UninterpretedOption<'a, Impl = Self>;
    type Location: Location<'a, Impl = Self>;
    type ReservedRanges: ReservedRanges<'a, Impl = Self>;
    type ReservedRange: ReservedRange<'a, Impl = Self>;
    type EnumReservedRanges: EnumReservedRanges<'a, Impl = Self>;
    type EnumReservedRange: EnumReservedRange<'a, Impl = Self>;

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
