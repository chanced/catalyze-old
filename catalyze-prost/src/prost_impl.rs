use catalyze::proto::{EnumDescriptor, FileDescriptor, MethodDescriptor};

use crate::proto::*;
impl Integration for Prost {
    type FileDescriptor = FileDescriptor;
    type MessageDescriptor = MessageDescriptor;
    type EnumDescriptor = EnumDescriptor;
    type EnumValueDescriptor = EnumValueDescriptor;
    type FieldDescriptor = FieldDescriptor;
    type OneofDescriptor = OneofDescriptor;
    type ServiceDescriptor = ServiceDescriptor;

    type MethodDescriptor = MethodDescriptor;
}
