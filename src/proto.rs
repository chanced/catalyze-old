mod enums;
pub mod iter;
pub use enums::*;

use lazy_static::lazy_static;
use std::{marker::PhantomData, slice};

lazy_static! {
    static ref DEFAULT_FILE_OPTIONS: prost_types::FileOptions = prost_types::FileOptions::default();
    static ref DEFAULT_MESSAGE_OPTIONS: prost_types::MessageOptions =
        prost_types::MessageOptions::default();
    static ref DEFAULT_ONEOF_OPTIONS: prost_types::OneofOptions =
        prost_types::OneofOptions::default();
    static ref DEFAULT_FIELD_OPTIONS: prost_types::FieldOptions =
        prost_types::FieldOptions::default();
    static ref DEFAULT_SERVICE_OPTIONS: prost_types::ServiceOptions =
        prost_types::ServiceOptions::default();
    static ref DEFAULT_METHOD_OPTIONS: prost_types::MethodOptions =
        prost_types::MethodOptions::default();
    static ref DEFAULT_ENUM_OPTIONS: prost_types::EnumOptions = prost_types::EnumOptions::default();
    static ref DEFAULT_ENUM_VALUE_OPTIONS: prost_types::EnumValueOptions =
        prost_types::EnumValueOptions::default();
    static ref DEFAULT_SOURCE_CODE_INFO: prost_types::SourceCodeInfo =
        prost_types::SourceCodeInfo::default();
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FileDescriptor<'a> {
    desc: &'a prost_types::FileDescriptorProto,
}

#[derive(Debug)]
/// Describes a complete .proto file.
pub struct FileDescriptor<'a> {
    desc: &'a prost_types::FileDescriptorProto,
}
impl<'a> interface::FileDescriptor<'a, Prost> for FileDescriptor<'a> {
    fn name(&self) -> &'a str {
        self.desc.name()
    }

    fn package(&self) -> String {
        self.desc.package().to_string()
    }

    fn dependencies(&self) -> slice::Iter<String> {
        self.desc.dependency.iter()
    }

    fn public_dependencies(&self) -> slice::Iter<i32> {
        self.desc.public_dependency.iter()
    }

    fn messages(&self) -> <Prost as Interface>::MessageDescriptorIter {}

    fn enums(&self) -> <Prost as Interface>::EnumDescriptorIter {
        todo!()
    }

    fn services(&self) -> <Prost as Interface>::ServiceDescriptorIter {
        todo!()
    }

    fn options(&self) -> <Prost as Interface>::FileOptions {
        todo!()
    }

    fn source_code_info(&self) -> <Prost as Interface>::SourceCodeInfo {
        todo!()
    }

    fn syntax(&self) -> enums::Syntax {
        todo!()
    }
}

// impl<'a> FileDescriptor<'a> {
//     /// file name, relative to root of source tree
//     pub fn name(&self) -> &'a str {}
//     /// e.g. "foo", "foo.bar", etc.
//     pub fn package(&self) -> &'a str {}
//     /// Names of files imported by this file.
//     pub fn dependencies(&self) -> slice::Iter<String> {
//
//     }

//     /// Indexes of the public imported files in the dependency list.
//     pub fn public_dependencies(&self) -> slice::Iter<i32> {
//         self.desc.public_dependency.iter()
//     }
//     /// All top-level `Message` definitions in this file.
//     pub fn messages(&self) -> MessageDescriptorIter<'a> {
//         (&self.desc.message_type).into()
//     }

//     pub fn comments(&self) -> CommentsIter<'a> {
//         self.source_code_info().into()
//     }

//     // All top-level `Enum` definitions in this file.
//     pub fn enums(&self) -> EnumDescriptorIter<'a> {
//         (&self.desc.enum_type).into()
//     }

//     /// All top-level `Service` definitions in this file.
//     pub fn services(&self) -> ServiceDescriptorIter<'a> {
//         (&self.desc.service).into()
//     }
//     pub fn extensions(&self) -> FieldDescriptorIter<'a> {
//         (&self.desc.extension).into()
//     }

//     pub fn options(&self) -> FileOptions<'a> {
//         self.desc.options.as_ref().into()
//     }
//     /// This field contains optional information about the original source code.
//     /// You may safely remove this entire field without harming runtime
//     /// functionality of the descriptors -- the information is needed only by
//     /// development tools.
//     pub fn source_code_info(&self) -> SourceCodeInfo<'a> {
//         self.desc.source_code_info.as_ref().into()
//     }
//     /// The syntax of the proto file.
//     /// The supported values are "proto2" and "proto3".
//     pub fn syntax(&self) -> Syntax {
//         self.desc.syntax().into()
//     }
// }
// impl<'a> From<&'a prost_types::FileDescriptorProto> for FileDescriptor<'a> {
//     fn from(desc: &'a prost_types::FileDescriptorProto) -> Self {
//         Self { desc }
//     }
// }
// impl<'a> Copy for FileDescriptor<'a> {}
// impl<'a> Clone for FileDescriptor<'a> {
//     fn clone(&self) -> Self {
//         Self { desc: self.desc }
//     }
// }

// /// Describes a message type.
// #[derive(Debug)]
// pub struct MessageDescriptor<'a> {
//     desc: &'a prost_types::DescriptorProto,
//     u: PhantomData<U>,
// }
// impl<'a> MessageDescriptor<'a> {
//     pub fn name(&self) -> &'a str {
//         self.desc.name()
//     }
//     pub fn fields(&self) -> FieldDescriptorIter<'a> {
//         let fields = &self.desc.field;
//         fields.into()
//     }

//     pub fn extensions(&self) -> FieldDescriptorIter<'a> {
//         (&self.desc.extension).into()
//     }

//     pub fn nested_messages(&self) -> MessageDescriptorIter<'a> {
//         (&self.desc.nested_type).into()
//     }

//     pub fn enums(&self) -> EnumDescriptorIter<'a> {
//         (&self.desc.enum_type).into()
//     }

//     pub fn extension_ranges(&self) -> ExtensionRanges<'a> {
//         (&self.desc.extension_range).into()
//     }

//     pub fn oneofs(&self) -> OneofDescriptorIter<'a> {
//         (&self.desc.oneof_decl).into()
//     }
//     pub fn options(&self) -> MessageOptions<'a> {
//         self.desc.options.as_ref().into()
//     }
//     pub fn reserved_ranges(&self) -> ReservedRanges<'a> {
//         (&self.desc.reserved_range).into()
//     }
//     /// Reserved field names, which may not be used by fields in the same message.
//     /// A given name may only be reserved once.
//     pub fn reserved_names(&self) -> slice::Iter<String> {
//         self.desc.reserved_name.iter()
//     }
// }
// impl<'a> From<&'a prost_types::DescriptorProto> for MessageDescriptor<'a> {
//     fn from(desc: &'a prost_types::DescriptorProto) -> Self {
//         Self { desc }
//     }
// }
// impl<'a> Copy for MessageDescriptor<'a> {}
// impl<'a> Clone for MessageDescriptor<'a> {
//     fn clone(&self) -> Self {
//         Self { desc: self.desc }
//     }
// }
// #[derive(Debug)]
// pub struct EnumDescriptor<'a> {
//     desc: &'a prost_types::EnumDescriptorProto,
//     u: PhantomData<U>,
// }
// impl<'a> EnumDescriptor<'a> {
//     pub fn name(&self) -> &'a str {
//         self.desc.name()
//     }
//     pub fn values(&self) -> EnumValueDescriptorIter<'a> {
//         (&self.desc.value).into()
//     }
//     pub fn options(&self) -> EnumOptions<'a> {
//         self.desc.options.as_ref().into()
//     }
//     /// Range of reserved numeric values. Reserved numeric values may not be used
//     /// by enum values in the same enum declaration. Reserved ranges may not
//     /// overlap.
//     pub fn reserved_ranges(&self) -> EnumReservedRanges<'a> {
//         (&self.desc.reserved_range).into()
//     }
//     pub fn reserved_names(&self) -> slice::Iter<String> {
//         self.desc.reserved_name.iter()
//     }
// }
// impl<'a> Copy for EnumDescriptor<'a> {}
// impl<'a> Clone for EnumDescriptor<'a> {
//     fn clone(&self) -> Self {
//         Self { desc: self.desc }
//     }
// }
// impl<'a> From<&'a prost_types::EnumDescriptorProto> for EnumDescriptor<'a> {
//     fn from(desc: &'a prost_types::EnumDescriptorProto) -> Self {
//         EnumDescriptor { desc }
//     }
// }

// #[derive(Debug)]
// pub struct EnumValueDescriptor<'a> {
//     desc: &'a prost_types::EnumValueDescriptorProto,
//     u: PhantomData<U>,
// }
// impl<'a> EnumValueDescriptor<'a> {
//     pub fn name(&self) -> &'a str {
//         self.desc.name()
//     }
//     pub fn number(&self) -> i32 {
//         self.desc.number()
//     }
//     pub fn options(&self) -> EnumValueOptions<'a> {
//         self.desc.options.as_ref().into()
//     }
// }
// impl<'a> Copy for EnumValueDescriptor<'a> {}
// impl<'a> Clone for EnumValueDescriptor<'a> {
//     fn clone(&self) -> Self {
//         Self { desc: self.desc }
//     }
// }
// impl<'a> From<&'a prost_types::EnumValueDescriptorProto> for EnumValueDescriptor<'a> {
//     fn from(desc: &'a prost_types::EnumValueDescriptorProto) -> Self {
//         Self { desc }
//     }
// }

// /// Describes a field within a message.
// #[derive(Debug)]
// pub struct FieldDescriptor<'a> {
//     desc: &'a prost_types::FieldDescriptorProto,
//     u: PhantomData<U>,
// }

// impl<'a> FieldDescriptor<'a> {
//     pub fn name(&self) -> &str {
//         self.desc.name()
//     }
//     pub fn number(&self) -> i32 {
//         self.desc.number()
//     }
//     pub fn label(&self) -> Label {
//         Label::from(self.desc.label())
//     }

//     pub fn well_known_type(&self) -> Option<WellKnownType> {
//         todo!()
//     }

//     pub fn is_well_known_type(&self) -> bool {
//         self.well_known_type().is_some()
//     }

//     /// If type_name is set, this need not be set.  If both this and type_name
//     /// are set, this must be one of Enum, Message or Group.
//     pub fn r#type(&self) -> Type {
//         Type::from(self.desc.r#type())
//     }

//     pub fn is_embed(&self) -> bool {
//         matches!(self.r#type(), Type::Message)
//     }

//     pub fn is_enum(&self) -> bool {
//         matches!(self.r#type(), Type::Enum)
//     }

//     pub fn is_scalar(&self) -> bool {
//         matches!(self.r#type(), Type::Scalar(_))
//     }

//     pub fn is_double(&self) -> bool {
//         matches!(self.r#type(), Type::Scalar(Scalar::Double))
//     }
//     pub fn is_float(&self) -> bool {
//         matches!(self.r#type(), Type::Scalar(Scalar::Float))
//     }

//     pub fn is_int64(&self) -> bool {
//         matches!(self.r#type(), Type::Scalar(Scalar::Int64))
//     }

//     pub fn is_uint64(&self) -> bool {
//         matches!(self.r#type(), Type::Scalar(Scalar::Uint64))
//     }

//     pub fn is_int32(&self) -> bool {
//         matches!(self.r#type(), Type::Scalar(Scalar::Int32))
//     }

//     pub fn is_fixed64(&self) -> bool {
//         matches!(self.r#type(), Type::Scalar(Scalar::Fixed64))
//     }

//     pub fn is_fixed32(&self) -> bool {
//         matches!(self.r#type(), Type::Scalar(Scalar::Fixed32))
//     }

//     pub fn is_bool(&self) -> bool {
//         matches!(self.r#type(), Type::Scalar(Scalar::Bool))
//     }

//     pub fn is_string(&self) -> bool {
//         matches!(self.r#type(), Type::Scalar(Scalar::String))
//     }

//     pub fn is_bytes(&self) -> bool {
//         matches!(self.r#type(), Type::Scalar(Scalar::Bytes))
//     }

//     pub fn is_uint32(&self) -> bool {
//         matches!(self.r#type(), Type::Scalar(Scalar::Uint32))
//     }

//     pub fn is_sfixed32(&self) -> bool {
//         matches!(self.r#type(), Type::Scalar(Scalar::Sfixed32))
//     }

//     pub fn is_sfixed64(&self) -> bool {
//         matches!(self.r#type(), Type::Scalar(Scalar::Sfixed64))
//     }

//     pub fn is_sint32(&self) -> bool {
//         matches!(self.r#type(), Type::Scalar(Scalar::Sint32))
//     }

//     pub fn is_sint64(&self) -> bool {
//         matches!(self.r#type(), Type::Scalar(Scalar::Sint64))
//     }
//     pub fn is_message(&self) -> bool {
//         matches!(self.r#type(), Type::Message)
//     }

//     pub fn is_repeated(&self) -> bool {
//         self.label() == Label::Repeated
//     }

//     /// alias for `r#type`
//     ///
//     /// If type_name is set, this need not be set.  If both this and type_name
//     /// are set, this must be one of Enum, Message or Group.
//     pub fn proto_type(&self) -> Type {
//         self.r#type()
//     }

//     /// For message and enum types, this is the name of the type.  If the name
//     /// starts with a '.', it is fully-qualified.  Otherwise, C++-like scoping
//     /// rules are used to find the type (i.e. first the nested types within this
//     /// message are searched, then within the parent, on up to the root
//     /// namespace).
//     pub fn type_name(&self) -> &str {
//         self.desc.type_name()
//     }

//     /// For extensions, this is the name of the type being extended.  It is
//     /// resolved in the same manner as `proto_type_name`.
//     pub fn extendee(&self) -> &str {
//         self.desc.extendee()
//     }
//     /// For numeric types, contains the original text representation of the value.
//     /// For booleans, "true" or "false".
//     /// For strings, contains the default text contents (not escaped in any way).
//     /// For bytes, contains the C escaped value.  All bytes >= 128 are escaped.
//     pub fn default_value(&self) -> &str {
//         self.desc.default_value()
//     }
//     /// If set, gives the index of a oneof in the containing type's oneof_decl
//     /// list.
//     ///
//     /// This field is a member of that oneof.
//     pub fn oneof_index(&self) -> i32 {
//         self.desc.oneof_index()
//     }

//     /// JSON name of this field. The value is set by protocol compiler. If the
//     /// user has set a "json_name" option on this field, that option's value
//     /// will be used. Otherwise, it's deduced from the field's name by converting
//     /// it to camelCase.
//     pub fn json_name(&self) -> &str {
//         self.desc.json_name()
//     }
//     pub fn options(&self) -> FieldOptions<'a> {
//         self.desc.options.as_ref().into()
//     }
//     /// If true, this is a proto3 "optional". When a proto3 field is optional, it
//     /// tracks presence regardless of field type.
//     ///
//     /// When proto3_optional is true, this field must be belong to a oneof to
//     /// signal to old proto3 clients that presence is tracked for this field. This
//     /// oneof is known as a "synthetic" oneof, and this field must be its sole
//     /// member (each proto3 optional field gets its own synthetic oneof). Synthetic
//     /// oneofs exist in the descriptor only, and do not generate any API. Synthetic
//     /// oneofs must be ordered after all "real" oneofs.
//     ///
//     /// For message fields, proto3_optional doesn't create any semantic change,
//     /// since non-repeated message fields always track presence. However it still
//     /// indicates the semantic detail of whether the user wrote "optional" or not.
//     /// This can be useful for round-tripping the .proto file. For consistency we
//     /// give message fields a synthetic oneof also, even though it is not required
//     /// to track presence. This is especially important because the parser can't
//     /// tell if a field is a message or an enum, so it must always create a
//     /// synthetic oneof.
//     ///
//     /// Proto2 optional fields do not set this flag, because they already indicate
//     /// optional with `LABEL_OPTIONAL`.
//     pub fn proto3_optional(&self) -> bool {
//         self.desc.proto3_optional()
//     }
//     /// returns `true` if:
//     ///
//     /// - `syntax` is `Syntax::Proto3` and `proto3_optional` is `true`
//     /// - `syntax` is `Syntax::Proto2` and `label` is `Label::Optional`.
//     pub fn is_marked_optional(&self, syntax: Syntax) -> bool {
//         match syntax {
//             Syntax::Proto2 => self.label() == Label::Optional,
//             Syntax::Proto3 => self.proto3_optional(),
//         }
//     }
//     pub fn is_required(&self, syntax: Syntax) -> bool {
//         syntax.supports_required_prefix() && self.label() == Label::Required
//     }
// }
// impl<'a> From<&'a prost_types::FieldDescriptorProto> for FieldDescriptor<'a> {
//     fn from(desc: &'a prost_types::FieldDescriptorProto) -> Self {
//         Self { desc }
//     }
// }
// impl<'a> Copy for FieldDescriptor<'a> {}
// impl<'a> Clone for FieldDescriptor<'a> {
//     fn clone(&self) -> Self {
//         Self { desc: self.desc }
//     }
// }

// /// Describes a service.
// #[derive(Debug)]
// pub struct ServiceDescriptor<'a> {
//     desc: &'a prost_types::ServiceDescriptorProto,
//     u: PhantomData<U>,
// }
// impl<'a> ServiceDescriptor<'a> {
//     pub fn name(&self) -> &'a str {
//         self.desc.name()
//     }
//     pub fn options(&self) -> ServiceOptions<'a> {
//         self.desc.options.as_ref().into()
//     }
//     pub fn methods(&self) -> MethodDescriptorIter<'a> {
//         (&self.desc.method).into()
//     }
// }
// impl<'a> From<&'a prost_types::ServiceDescriptorProto> for ServiceDescriptor<'a> {
//     fn from(desc: &'a prost_types::ServiceDescriptorProto) -> Self {
//         Self { desc }
//     }
// }
// impl<'a> Copy for ServiceDescriptor<'a> {}
// impl<'a> Clone for ServiceDescriptor<'a> {
//     fn clone(&self) -> Self {
//         Self { desc: self.desc }
//     }
// }

// /// Describes a method.
// #[derive(Debug)]
// pub struct MethodDescriptor<'a> {
//     desc: &'a prost_types::MethodDescriptorProto,
//     u: PhantomData<U>,
// }
// impl<'a> MethodDescriptor<'a> {
//     pub fn name(&self) -> &'a str {
//         self.desc.name()
//     }
//     /// Input type name.
//     ///
//     /// These are resolved in the same way as
//     /// `FieldDescriptor.type_name`, but must refer to a message type
//     pub fn input_type(&self) -> &'a str {
//         self.desc.input_type()
//     }
//     /// Output type name.
//     ///
//     /// These are resolved in the same way as
//     /// `FieldDescriptor.type_name`, but must refer to a message type
//     pub fn output_type(&self) -> &'a str {
//         self.desc.output_type()
//     }
//     /// Identifies if client streams multiple client messages
//     pub fn client_streaming(&self) -> bool {
//         self.desc.client_streaming()
//     }
//     /// Identifies if server streams multiple server messages
//     pub fn server_streaming(&self) -> bool {
//         self.desc.server_streaming()
//     }
//     pub fn options(&self) -> MethodOptions<'a> {
//         self.desc.options.as_ref().into()
//     }
// }
// impl<'a> From<&'a prost_types::MethodDescriptorProto> for MethodDescriptor<'a> {
//     fn from(desc: &'a prost_types::MethodDescriptorProto) -> Self {
//         Self { desc }
//     }
// }
// impl<'a> Copy for MethodDescriptor<'a> {}
// impl<'a> Clone for MethodDescriptor<'a> {
//     fn clone(&self) -> Self {
//         Self { desc: self.desc }
//     }
// }

// /// Describes a oneof.
// #[derive(Debug)]
// pub struct OneofDescriptor<'a> {
//     desc: &'a prost_types::OneofDescriptorProto,
//     u: PhantomData<U>,
// }
// impl<'a> OneofDescriptor<'a> {
//     pub fn name(&self) -> &'a str {
//         self.desc.name()
//     }
//     pub fn options(&self) -> OneofOptions<'a> {
//         self.desc.options.as_ref().into()
//     }
// }
// impl<'a> From<&'a prost_types::OneofDescriptorProto> for OneofDescriptor<'a> {
//     fn from(desc: &'a prost_types::OneofDescriptorProto) -> Self {
//         Self { desc }
//     }
// }
// impl<'a> Copy for OneofDescriptor<'a> {}
// impl<'a> Clone for OneofDescriptor<'a> {
//     fn clone(&self) -> Self {
//         Self { desc: self.desc }
//     }
// }

// // ===================================================================
// // Options

// // Each of the definitions above may have "options" attached.  These are
// // just annotations which may cause code to be generated slightly differently
// // or may contain hints for code that manipulates protocol messages.
// //
// // Clients may define custom options as extensions of the *Options messages.
// // These extensions may not yet be known at parsing time, so the parser cannot
// // store the values in them.  Instead it stores them in a field in the *Options
// // message called uninterpreted_option. This field must have the same name
// // across all *Options messages. We then use this field to populate the
// // extensions when we build a descriptor, at which point all protos have been
// // parsed and so all extensions are known.
// //
// // Extension numbers for custom options may be chosen as follows:
// // * For options which will only be used within a single application or
// //   organization, or for experimental options, use field numbers 50000
// //   through 99999.  It is up to you to ensure that you do not use the
// //   same number for multiple options.
// // * For options which will be published and used publicly by multiple
// //   independent entities, e-mail protobuf-global-extension-registry@google.com
// //   to reserve extension numbers. Simply provide your project name (e.g.
// //   Objective-C plugin) and your project website (if available) -- there's no
// //   need to explain how you intend to use them. Usually you only need one
// //   extension number. You can declare multiple options with only one extension
// //   number by putting them in a sub-message. See the Custom Options section of
// //   the docs for examples:
// //   <https://developers.google.com/protocol-buffers/docs/proto#options>
// //   If this turns out to be popular, a web service will be set up
// //   to automatically assign option numbers.

// #[derive(Debug)]
// pub struct FileOptions<'a> {
//     opts: Option<&'a prost_types::FileOptions>,
//     u: PhantomData<U>,
// }
// impl<'a> From<Option<&'a prost_types::FileOptions>> for FileOptions<'a> {
//     fn from(opts: Option<&'a prost_types::FileOptions>) -> Self {
//         Self { opts }
//     }
// }
// impl<'a> FileOptions<'a> {
//     /// Java package where classes generated from this .proto will be
//     /// placed.  By default, the proto package is used, but this is often
//     /// inappropriate because proto packages do not normally start with backwards
//     /// domain names.
//     pub fn java_package(&self) -> &str {
//         self.opts().java_package()
//     }
//     /// If set, all the classes from the .proto file are wrapped in a single
//     /// outer class with the given name.  This applies to both Proto1
//     /// (equivalent to the old "--one_java_file" option) and Proto2 (where
//     /// a .proto always translates to a single class, but you may want to
//     /// explicitly choose the class name).
//     pub fn java_outer_classname(&self) -> &str {
//         self.opts().java_outer_classname()
//     }

//     /// If set true, then the Java code generator will generate a separate .java
//     /// file for each top-level message, enum, and service defined in the .proto
//     /// file.  Thus, these types will *not* be nested inside the outer class
//     /// named by java_outer_classname.  However, the outer class will still be
//     /// generated to contain the file's getDescriptor() method as well as any
//     /// top-level extensions defined in the file.
//     pub fn java_multiple_files(&self) -> bool {
//         self.opts().java_multiple_files()
//     }

//     /// If set true, then the Java2 code generator will generate code that
//     /// throws an exception whenever an attempt is made to assign a non-UTF-8
//     /// byte sequence to a string field.
//     /// Message reflection will do the same.
//     /// However, an extension field still accepts non-UTF-8 byte sequences.
//     /// This option has no effect on when used with the lite runtime.
//     pub fn java_string_check_utf8(&self) -> bool {
//         self.opts().java_string_check_utf8()
//     }
//     /// Generated classes can be optimized for speed or code size.
//     pub fn optimize_for(&self) -> OptimizeMode {
//         self.opts().optimize_for().into()
//     }
//     /// Sets the Go package where structs generated from this .proto will be
//     /// placed. If omitted, the Go package will be derived from the following:
//     ///   - The basename of the package import path, if provided.
//     ///   - Otherwise, the package statement in the .proto file, if present.
//     ///   - Otherwise, the basename of the .proto file, without extension.
//     pub fn go_package(&self) -> &str {
//         self.opts().go_package()
//     }
//     /// Should generic services be generated in each language?  "Generic" services
//     /// are not specific to any particular RPC system.  They are generated by the
//     /// main code generators in each language (without additional plugins).
//     /// Generic services were the only kind of service generation supported by
//     /// early versions of google.protobuf.
//     ///
//     /// Generic services are now considered deprecated in favor of using plugins
//     /// that generate code specific to your particular RPC system.  Therefore,
//     /// these default to false.  Old code which depends on generic services should
//     /// explicitly set them to true.
//     pub fn cc_generic_services(&self) -> bool {
//         self.opts().cc_generic_services()
//     }

//     pub fn java_generic_services(&self) -> bool {
//         self.opts().java_generic_services()
//     }

//     pub fn py_generic_services(&self) -> bool {
//         self.opts().py_generic_services()
//     }

//     pub fn php_generic_services(&self) -> bool {
//         self.opts().php_generic_services()
//     }

//     /// Is this file deprecated?
//     /// Depending on the target platform, this can emit Deprecated annotations
//     /// for everything in the file, or it will be completely ignored; in the very
//     /// least, this is a formalization for deprecating files.
//     pub fn deprecated(&self) -> bool {
//         self.opts().deprecated()
//     }
//     pub fn is_deprecated(&self) -> bool {
//         self.opts().deprecated()
//     }
//     /// Enables the use of arenas for the proto messages in this file. This applies
//     /// only to generated classes for C++.
//     pub fn cc_enable_arenas(&self) -> bool {
//         self.opts().cc_enable_arenas()
//     }
//     /// Sets the objective c class prefix which is prepended to all objective c
//     /// generated classes from this .proto. There is no default.
//     pub fn objc_class_prefix(&self) -> &str {
//         self.opts().objc_class_prefix()
//     }

//     /// Namespace for generated classes; defaults to the package.
//     pub fn csharp_namespace(&self) -> &str {
//         self.opts().csharp_namespace()
//     }
//     /// By default Swift generators will take the proto package and CamelCase it
//     /// replacing '.' with underscore and use that to prefix the types/symbols
//     /// defined. When this options is provided, they will use this value instead
//     /// to prefix the types/symbols defined.
//     pub fn swift_prefix(&self) -> &str {
//         self.opts().swift_prefix()
//     }

//     /// Sets the php class prefix which is prepended to all php generated classes
//     /// from this .proto. Default is empty.
//     pub fn php_class_prefix(&self) -> &str {
//         self.opts().php_class_prefix()
//     }

//     /// Use this option to change the namespace of php generated classes. Default
//     /// is empty. When this option is empty, the package name will be used for
//     /// determining the namespace.
//     pub fn php_namespace(&self) -> &str {
//         self.opts().php_namespace()
//     }

//     /// Use this option to change the namespace of php generated metadata classes.
//     /// Default is empty. When this option is empty, the proto file name will be
//     /// used for determining the namespace.
//     pub fn php_metadata_namespace(&self) -> &str {
//         self.opts().php_metadata_namespace()
//     }
//     /// Use this option to change the package of ruby generated classes. Default
//     /// is empty. When this option is not set, the package name will be used for
//     /// determining the ruby package.
//     pub fn ruby_package(&self) -> &str {
//         self.opts().ruby_package()
//     }
//     /// The parser stores options it doesn't recognize here.
//     /// See the documentation for the "Options" section above.
//     pub fn uninterpreted_options(&self) -> UninterpretedOptions<'a> {
//         (&self.opts().uninterpreted_option).into()
//     }
//     fn opts(&self) -> &'a prost_types::FileOptions {
//         self.opts.unwrap_or(&DEFAULT_FILE_OPTIONS)
//     }
// }
// impl<'a> Copy for FileOptions<'a> {}
// impl<'a> Clone for FileOptions<'a> {
//     fn clone(&self) -> Self {
//         FileOptions { opts: self.opts }
//     }
// }

// #[derive(Debug)]
// pub struct EnumValueOptions<'a> {
//     opts: Option<&'a prost_types::EnumValueOptions>,
//     u: PhantomData<&'a U>,
// }
// impl<'a> EnumValueOptions<'a> {
//     /// Is this enum value deprecated?
//     /// Depending on the target platform, this can emit Deprecated annotations
//     /// for the enum value, or it will be completely ignored; in the very least,
//     /// this is a formalization for deprecating enum values.
//     pub fn deprecated(&self) -> bool {
//         self.opts().deprecated()
//     }
//     pub fn is_deprecated(&self) -> bool {
//         self.opts().deprecated()
//     }
//     /// Options not recognized by the parser.
//     pub fn uninterpreted_options(&self) -> UninterpretedOptions<'a> {
//         (&self.opts().uninterpreted_option).into()
//     }
//     fn opts(&self) -> &'a prost_types::EnumValueOptions {
//         self.opts.unwrap_or(&DEFAULT_ENUM_VALUE_OPTIONS)
//     }
// }
// impl<'a> From<Option<&'a prost_types::EnumValueOptions>> for EnumValueOptions<'a> {
//     fn from(opts: Option<&'a prost_types::EnumValueOptions>) -> Self {
//         Self { opts }
//     }
// }
// impl<'a> Copy for EnumValueOptions<'a> {}
// impl<'a> Clone for EnumValueOptions<'a> {
//     fn clone(&self) -> Self {
//         Self { opts: self.opts }
//     }
// }

// #[derive(Debug)]
// pub struct MessageOptions<'a> {
//     opts: Option<&'a prost_types::MessageOptions>,
//     u: PhantomData<U>,
// }
// impl<'a> MessageOptions<'a> {
//     /// Set true to use the old proto1 MessageSet wire format for extensions.
//     /// This is provided for backwards-compatibility with the MessageSet wire
//     /// format.  You should not use this for any other reason:  It's less
//     /// efficient, has fewer features, and is more complicated.
//     ///
//     /// The message must be defined exactly as follows:
//     ///   message Foo {
//     ///     option message_set_wire_format = true;
//     ///     extensions 4 to max;
//     ///   }
//     /// Note that the message cannot have any defined fields; MessageSets only
//     /// have extensions.
//     ///
//     /// All extensions of your type must be singular messages; e.g. they cannot
//     /// be int32s, enums, or repeated messages.
//     ///
//     /// Because this is an option, the above two restrictions are not enforced by
//     /// the protocol compiler.
//     pub fn message_set_wire_format(&self) -> bool {
//         self.opts().message_set_wire_format()
//     }
//     /// Whether the message is an automatically generated map entry type for the
//     /// maps field.
//     ///
//     /// For maps fields:
//     ///     map<KeyType, ValueType> map_field = 1;
//     /// The parsed descriptor looks like:
//     ///     message MapFieldEntry {
//     ///         option map_entry = true;
//     ///         optional KeyType key = 1;
//     ///         optional ValueType value = 2;
//     ///     }
//     ///     repeated MapFieldEntry map_field = 1;
//     ///
//     /// Implementations may choose not to generate the map_entry=true message, but
//     /// use a native map in the target language to hold the keys and values.
//     /// The reflection APIs in such implementations still need to work as
//     /// if the field is a repeated message field.
//     ///
//     /// NOTE: Do not set the option in .proto files. Always use the maps syntax
//     /// instead. The option should only be implicitly set by the proto compiler
//     /// parser.
//     pub fn map_entry(&self) -> bool {
//         self.opts().map_entry()
//     }

//     pub fn is_map_entry(&self) -> bool {
//         self.map_entry()
//     }

//     pub fn deprecated(&self) -> bool {
//         self.opts().deprecated()
//     }
//     pub fn is_deprecated(&self) -> bool {
//         self.opts().deprecated()
//     }
//     pub fn no_standard_descriptor_accessor(&self) -> bool {
//         self.opts().no_standard_descriptor_accessor()
//     }
//     /// The parser stores options it doesn't recognize here. See above.
//     pub fn uninterpreted_option(&self) -> UninterpretedOptions<'a> {
//         (&self.opts().uninterpreted_option).into()
//     }
//     fn opts(&self) -> &'a prost_types::MessageOptions {
//         self.opts.unwrap_or(&DEFAULT_MESSAGE_OPTIONS)
//     }
// }
// impl<'a> From<Option<&'a prost_types::MessageOptions>> for MessageOptions<'a> {
//     fn from(opts: Option<&'a prost_types::MessageOptions>) -> Self {
//         MessageOptions { opts }
//     }
// }
// impl<'a> Copy for MessageOptions<'a> {}
// impl<'a> Clone for MessageOptions<'a> {
//     fn clone(&self) -> Self {
//         MessageOptions { opts: self.opts }
//     }
// }

// #[derive(Debug)]
// pub struct FieldOptions<'a> {
//     opts: Option<&'a prost_types::FieldOptions>,
//     u: PhantomData<U>,
// }
// impl<'a> FieldOptions<'a> {
//     pub fn new(opts: Option<&'a prost_types::FieldOptions>) -> Self {
//         Self { opts }
//     }
//     /// The ctype option instructs the C++ code generator to use a different
//     /// representation of the field than it normally would.  See the specific
//     /// options below.  This option is not yet implemented in the open source
//     /// release -- sorry, we'll try to include it in a future version!
//     pub fn c_type(&self) -> CType {
//         CType::from(self.opts().ctype())
//     }
//     /// The packed option can be enabled for repeated primitive fields to enable
//     /// a more efficient representation on the wire. Rather than repeatedly
//     /// writing the tag and type for each element, the entire array is encoded as
//     /// a single length-delimited blob. In proto3, only explicit setting it to
//     /// false will avoid using packed encoding.
//     pub fn packed(&self) -> bool {
//         self.opts().packed()
//     }
//     /// The jstype option determines the JavaScript type used for values of the
//     /// field.  The option is permitted only for 64 bit integral and fixed types
//     /// (int64, uint64, sint64, fixed64, sfixed64).  A field with jstype JS_STRING
//     /// is represented as JavaScript string, which avoids loss of precision that
//     /// can happen when a large value is converted to a floating point JavaScript.
//     /// Specifying JS_NUMBER for the jstype causes the generated JavaScript code to
//     /// use the JavaScript "number" type.  The behavior of the default option
//     /// JS_NORMAL is implementation dependent.
//     ///
//     /// This option is an enum to permit additional types to be added, e.g.
//     /// goog.math.Integer.
//     pub fn js_type(&self) -> JsType {
//         self.opts().jstype().into()
//     }
//     /// Should this field be parsed lazily?  Lazy applies only to message-type
//     /// fields.  It means that when the outer message is initially parsed, the
//     /// inner message's contents will not be parsed but instead stored in encoded
//     /// form.  The inner message will actually be parsed when it is first accessed.
//     ///
//     /// This is only a hint.  Implementations are free to choose whether to use
//     /// eager or lazy parsing regardless of the value of this option.  However,
//     /// setting this option true suggests that the protocol author believes that
//     /// using lazy parsing on this field is worth the additional bookkeeping
//     /// overhead typically needed to implement it.
//     ///
//     /// This option does not affect the public interface of any generated code;
//     /// all method signatures remain the same.  Furthermore, thread-safety of the
//     /// interface is not affected by this option; const methods remain safe to
//     /// call from multiple threads concurrently, while non-const methods continue
//     /// to require exclusive access.
//     ///
//     ///
//     /// Note that implementations may choose not to check required fields within
//     /// a lazy sub-message.  That is, calling IsInitialized() on the outer message
//     /// may return true even if the inner message has missing required fields.
//     /// This is necessary because otherwise the inner message would have to be
//     /// parsed in order to perform the check, defeating the purpose of lazy
//     /// parsing.  An implementation which chooses not to check required fields
//     /// must be consistent about it.  That is, for any particular sub-message, the
//     /// implementation must either *always* check its required fields, or *never*
//     /// check its required fields, regardless of whether or not the message has
//     /// been parsed.
//     pub fn is_lazy(&self) -> bool {
//         self.opts().lazy()
//     }
//     /// Is this field deprecated?
//     /// Depending on the target platform, this can emit Deprecated annotations
//     /// for accessors, or it will be completely ignored; in the very least, this
//     /// is a formalization for deprecating fields.
//     pub fn deprecated(&self) -> bool {
//         self.opts().deprecated()
//     }
//     pub fn is_deprecated(&self) -> bool {
//         self.opts().deprecated()
//     }
//     /// For Google-internal migration only. Do not use.
//     pub fn is_weak(&self) -> bool {
//         self.opts().weak()
//     }

//     /// Options the parser does not recognize.
//     pub fn uninterpreted_options(&self) -> UninterpretedOptions<'a> {
//         (&self.opts().uninterpreted_option).into()
//     }

//     fn opts(&self) -> &'a prost_types::FieldOptions {
//         self.opts.unwrap_or(&DEFAULT_FIELD_OPTIONS)
//     }
// }
// impl<'a> From<Option<&'a prost_types::FieldOptions>> for FieldOptions<'a> {
//     fn from(opts: Option<&'a prost_types::FieldOptions>) -> Self {
//         Self { opts }
//     }
// }
// impl<'a> Copy for FieldOptions<'a> {}
// impl<'a> Clone for FieldOptions<'a> {
//     fn clone(&self) -> Self {
//         Self { opts: self.opts }
//     }
// }

// #[derive(Debug)]
// pub struct EnumOptions<'a> {
//     opts: Option<&'a prost_types::EnumOptions>,
//     u: PhantomData<U>,
// }
// impl<'a> EnumOptions<'a> {
//     /// Is this enum deprecated?
//     /// Depending on the target platform, this can emit Deprecated annotations
//     /// for the enum, or it will be completely ignored; in the very least, this
//     /// is a formalization for deprecating enums.
//     pub fn deprecated(&self) -> bool {
//         self.opts().deprecated()
//     }
//     pub fn is_deprecated(&self) -> bool {
//         self.opts().deprecated()
//     }
//     /// Options not recognized by the parser.
//     pub fn uninterpreted_options(&self) -> UninterpretedOptions<'a> {
//         (&self.opts().uninterpreted_option).into()
//     }
//     /// Allows mapping different tag names to the same value.
//     pub fn allow_alias(&self) -> bool {
//         self.opts().allow_alias()
//     }
//     fn opts(&self) -> &'a prost_types::EnumOptions {
//         self.opts.unwrap_or(&DEFAULT_ENUM_OPTIONS)
//     }
// }
// impl<'a> From<Option<&'a prost_types::EnumOptions>> for EnumOptions<'a> {
//     fn from(opts: Option<&'a prost_types::EnumOptions>) -> Self {
//         Self { opts }
//     }
// }
// impl<'a> Copy for EnumOptions<'a> {}
// impl<'a> Clone for EnumOptions<'a> {
//     fn clone(&self) -> Self {
//         Self { opts: self.opts }
//     }
// }

// /// Options for a Service.
// ///
// /// Note: Field numbers 1 through 32 are reserved for Google's internal RPC
// /// framework.
// #[derive(Debug)]
// pub struct ServiceOptions<'a> {
//     opts: Option<&'a prost_types::ServiceOptions>,
//     u: PhantomData<U>,
// }
// impl<'a> ServiceOptions<'a> {
//     /// Is this service deprecated?
//     /// Depending on the target platform, this can emit Deprecated annotations
//     /// for the service, or it will be completely ignored; in the very least,
//     /// this is a formalization for deprecating services.
//     pub fn deprecated(&self) -> bool {
//         self.opts().deprecated()
//     }
//     pub fn is_deprecated(&self) -> bool {
//         self.opts().deprecated()
//     }
//     /// The parser stores options it doesn't recognize here. See above.
//     pub fn uninterpreted_options(&self) -> UninterpretedOptions<'a> {
//         (&self.opts().uninterpreted_option).into()
//     }
//     fn opts(&self) -> &'a prost_types::ServiceOptions {
//         self.opts.unwrap_or(&DEFAULT_SERVICE_OPTIONS)
//     }
// }
// impl<'a> From<Option<&'a prost_types::ServiceOptions>> for ServiceOptions<'a> {
//     fn from(opts: Option<&'a prost_types::ServiceOptions>) -> Self {
//         Self { opts }
//     }
// }
// impl<'a> Copy for ServiceOptions<'a> {}
// impl<'a> Clone for ServiceOptions<'a> {
//     fn clone(&self) -> Self {
//         Self { opts: self.opts }
//     }
// }

// /// Options for a Method.
// ///
// /// Note:  Field numbers 1 through 32 are reserved for Google's internal RPC
// /// framework.
// pub struct MethodOptions<'a> {
//     opts: Option<&'a prost_types::MethodOptions>,
//     u: PhantomData<U>,
// }
// impl<'a> MethodOptions<'a> {
//     // Note:  Field numbers 1 through 32 are reserved for Google's internal RPC
//     //   framework.  We apologize for hoarding these numbers to ourselves, but
//     //   we were already using them long before we decided to release Protocol
//     //   Buffers.

//     /// Is this method deprecated?
//     /// Depending on the target platform, this can emit Deprecated annotations
//     /// for the method, or it will be completely ignored; in the very least,
//     /// this is a formalization for deprecating methods.
//     pub fn deprecated(&self) -> bool {
//         self.opts().deprecated()
//     }
//     pub fn is_deprecated(&self) -> bool {
//         self.opts().deprecated()
//     }
//     /// The parser stores options it doesn't recognize here. See above.
//     pub fn uninterpreted_options(&self) -> UninterpretedOptions<'a> {
//         (&self.opts().uninterpreted_option).into()
//     }

//     /// Is this method side-effect-free (or safe in HTTP parlance), or idempotent,
//     /// or neither? HTTP based RPC implementation may choose GET verb for safe
//     /// methods, and PUT verb for idempotent methods instead of the default POST.
//     pub fn idempotency_level(&self) -> IdempotencyLevel {
//         self.opts().idempotency_level().into()
//     }
//     fn opts(&self) -> &'a prost_types::MethodOptions {
//         self.opts.unwrap_or(&DEFAULT_METHOD_OPTIONS)
//     }
// }
// impl<'a> From<Option<&'a prost_types::MethodOptions>> for MethodOptions<'a> {
//     fn from(opts: Option<&'a prost_types::MethodOptions>) -> Self {
//         Self { opts }
//     }
// }
// impl<'a> Copy for MethodOptions<'a> {}
// impl<'a> Clone for MethodOptions<'a> {
//     fn clone(&self) -> Self {
//         Self { opts: self.opts }
//     }
// }

// #[derive(Debug)]
// pub struct OneofOptions<'a> {
//     opts: Option<&'a prost_types::OneofOptions>,
//     u: PhantomData<U>,
// }
// impl<'a> OneofOptions<'a> {
//     /// The parser stores options it doesn't recognize here. See above.
//     pub fn uninterpreted_options(&self) -> UninterpretedOptions<'a> {
//         (&self.opts().uninterpreted_option).into()
//     }
//     pub fn opts(&self) -> &'a prost_types::OneofOptions {
//         self.opts.unwrap_or(&DEFAULT_ONEOF_OPTIONS)
//     }
// }
// impl<'a> From<Option<&'a prost_types::OneofOptions>> for OneofOptions<'a> {
//     fn from(opts: Option<&'a prost_types::OneofOptions>) -> Self {
//         Self { opts }
//     }
// }
// impl<'a> Copy for OneofOptions<'a> {}
// impl<'a> Clone for OneofOptions<'a> {
//     fn clone(&self) -> Self {
//         Self { opts: self.opts }
//     }
// }

// #[derive(Debug)]
// pub struct UninterpretedOptions<'a> {
//     pub(crate) opts: &'a [prost_types::UninterpretedOption],
//     u: PhantomData<U>,
// }
// impl<'a> UninterpretedOptions<'a> {
//     pub fn iter(&self) -> UninterpretedOptionsIter<'a> {
//         self.into()
//     }
// }
// impl<'a> IntoIterator for UninterpretedOptions<'a> {
//     type Item = UninterpretedOption<'a>;
//     type IntoIter = UninterpretedOptionsIter<'a>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.into()
//     }
// }
// impl<'a> From<&'a Vec<prost_types::UninterpretedOption>> for UninterpretedOptions<'a> {
//     fn from(opts: &'a Vec<prost_types::UninterpretedOption>) -> Self {
//         UninterpretedOptions { opts }
//     }
// }
// impl<'a> Copy for UninterpretedOptions<'a> {}
// impl<'a> Clone for UninterpretedOptions<'a> {
//     fn clone(&self) -> Self {
//         UninterpretedOptions { opts: self.opts }
//     }
// }

// /// A message representing a option the parser does not recognize. This only
// /// appears in options protos created by the compiler::Parser class.
// /// DescriptorPool resolves these when building Descriptor objects. Therefore,
// /// options protos in descriptor objects (e.g. returned by Descriptor::options(),
// /// or produced by Descriptor::CopyTo()) will never have UninterpretedOptions
// /// in them.
// #[derive(Debug)]
// pub struct UninterpretedOption<'a> {
//     opt: &'a prost_types::UninterpretedOption,
//     u: PhantomData<U>,
// }
// impl<'a> UninterpretedOption<'a> {
//     pub fn name_parts(&self) -> NameParts<'a> {
//         NameParts::from(&self.opt.name)
//     }
// }
// impl<'a> From<&'a prost_types::UninterpretedOption> for UninterpretedOption<'a> {
//     fn from(opt: &'a prost_types::UninterpretedOption) -> Self {
//         UninterpretedOption { opt }
//     }
// }
// impl<'a> Copy for UninterpretedOption<'a> {}
// impl<'a> Clone for UninterpretedOption<'a> {
//     fn clone(&self) -> Self {
//         UninterpretedOption { opt: self.opt }
//     }
// }

// /// Range of reserved tag numbers. Reserved tag numbers may not be used by
// /// fields or extension ranges in the same message. Reserved ranges may
// /// not overlap.
// #[derive(Debug)]
// pub struct ReservedRange<'a> {
//     range: &'a prost_types::descriptor_proto::ReservedRange,
//     u: PhantomData<U>,
// }
// impl<'a> ReservedRange<'a> {
//     /// Inclusive.
//     pub fn start(&self) -> i32 {
//         self.range.start()
//     }

//     /// Exclusive.
//     pub fn end(&self) -> i32 {
//         self.range.end()
//     }

//     pub fn in_range(&self, val: i32) -> bool {
//         self.start() <= val && val < self.end()
//     }
// }
// impl<'a> PartialEq for ReservedRange<'a> {
//     fn eq(&self, other: &Self) -> bool {
//         self.start() == other.start() && self.end() == other.end()
//     }
// }
// impl<'a> From<&'a prost_types::descriptor_proto::ReservedRange> for ReservedRange<'a> {
//     fn from(range: &'a prost_types::descriptor_proto::ReservedRange) -> Self {
//         ReservedRange { range }
//     }
// }
// impl<'a> Copy for ReservedRange<'a> {}
// impl<'a> Clone for ReservedRange<'a> {
//     fn clone(&self) -> Self {
//         ReservedRange { range: self.range }
//     }
// }

// #[derive(Debug)]
// pub struct ReservedRanges<'a> {
//     ranges: &'a [prost_types::descriptor_proto::ReservedRange],
//     u: PhantomData<U>,
// }
// impl<'a> IntoIterator for ReservedRanges<'a> {
//     type Item = ReservedRange<'a>;
//     type IntoIter = ReservedRangeIter<'a>;
//     fn into_iter(self) -> Self::IntoIter {
//         self.ranges.into()
//     }
// }
// impl<'a> ReservedRanges<'a> {
//     pub fn iter(&self) -> ReservedRangeIter<'a> {
//         self.ranges.into()
//     }
//     pub fn len(&self) -> usize {
//         self.ranges.len()
//     }
//     pub fn is_empty(&self) -> bool {
//         self.ranges.is_empty()
//     }
//     pub fn get(&self, index: usize) -> Option<ReservedRange<'a>> {
//         self.ranges.get(index).map(Into::into)
//     }
//     pub fn is_range_reserved(&self, min: i32, max: i32) -> bool {
//         self.iter().any(|r| r.start() <= min && r.end() >= max)
//     }
//     pub fn is_in_reserved_range(&self, num: i32) -> bool {
//         self.iter().any(|r| r.start() <= num && r.end() >= num)
//     }
// }
// impl<'a> From<&'a Vec<prost_types::descriptor_proto::ReservedRange>> for ReservedRanges<'a> {
//     fn from(ranges: &'a Vec<prost_types::descriptor_proto::ReservedRange>) -> Self {
//         ReservedRanges { ranges }
//     }
// }
// impl<'a> Copy for ReservedRanges<'a> {}
// impl<'a> Clone for ReservedRanges<'a> {
//     fn clone(&self) -> Self {
//         ReservedRanges {
//             ranges: self.ranges,
//         }
//     }
// }

// #[derive(Debug)]
// pub struct ExtensionRange<'a> {
//     range: &'a prost_types::descriptor_proto::ExtensionRange,
//     u: PhantomData<U>,
// }
// impl<'a> PartialEq for ExtensionRange<'a> {
//     fn eq(&self, other: &Self) -> bool {
//         self.range.start() == other.start() && self.end() == other.end()
//     }
// }
// impl<'a> ExtensionRange<'a> {
//     /// Inclusive.
//     pub fn start(&self) -> i32 {
//         self.range.start()
//     }
//     /// Exclusive.
//     pub fn end(&self) -> i32 {
//         self.range.end()
//     }
//     pub fn in_range(&self, val: i32) -> bool {
//         self.start() <= val && val < self.end()
//     }
// }
// impl<'a> From<&'a prost_types::descriptor_proto::ExtensionRange> for ExtensionRange<'a> {
//     fn from(range: &'a prost_types::descriptor_proto::ExtensionRange) -> Self {
//         ExtensionRange { range }
//     }
// }
// impl<'a> Copy for ExtensionRange<'a> {}
// impl<'a> Clone for ExtensionRange<'a> {
//     fn clone(&self) -> Self {
//         ExtensionRange { range: self.range }
//     }
// }

// #[derive(Debug)]
// pub struct ExtensionRanges<'a> {
//     ranges: &'a [prost_types::descriptor_proto::ExtensionRange],
//     u: PhantomData<U>,
// }
// impl<'a> ExtensionRanges<'a> {
//     pub fn iter(&self) -> ExtensionRangeIter<'a> {
//         self.ranges.into()
//     }
//     pub fn len(&self) -> usize {
//         self.ranges.len()
//     }
//     pub fn is_empty(&self) -> bool {
//         self.ranges.is_empty()
//     }
//     pub fn first(&self) -> Option<ExtensionRange<'a>> {
//         self.ranges.first().map(|r| r.into())
//     }
//     pub fn last(&self) -> Option<ExtensionRange<'a>> {
//         self.ranges.last().map(|r| r.into())
//     }
//     pub fn get(&self, n: usize) -> Option<ExtensionRange<'a>> {
//         self.ranges.get(n).map(|r| r.into())
//     }
// }
// impl<'a> IntoIterator for ExtensionRanges<'a> {
//     type Item = ExtensionRange<'a>;
//     type IntoIter = ExtensionRangeIter<'a>;
//     fn into_iter(self) -> Self::IntoIter {
//         self.iter()
//     }
// }
// impl<'a> From<&'a Vec<prost_types::descriptor_proto::ExtensionRange>> for ExtensionRanges<'a> {
//     fn from(ranges: &'a Vec<prost_types::descriptor_proto::ExtensionRange>) -> Self {
//         ExtensionRanges { ranges }
//     }
// }
// impl<'a> Copy for ExtensionRanges<'a> {}
// impl<'a> Clone for ExtensionRanges<'a> {
//     fn clone(&self) -> Self {
//         ExtensionRanges {
//             ranges: self.ranges,
//         }
//     }
// }

// /// Range of reserved numeric values. Reserved values may not be used by
// /// entries in the same enum. Reserved ranges may not overlap.
// ///
// /// Note that this is distinct from DescriptorProto.ReservedRange in that it
// /// is inclusive such that it can appropriately represent the entire int32
// /// domain.
// #[derive(Debug, PartialEq)]
// pub struct EnumReservedRange<'a> {
//     u: PhantomData<U>,
//     rr: &'a prost_types::enum_descriptor_proto::EnumReservedRange,
// }
// impl<'a> From<&'a prost_types::enum_descriptor_proto::EnumReservedRange> for EnumReservedRange<'a> {
//     fn from(r: &'a prost_types::enum_descriptor_proto::EnumReservedRange) -> Self {
//         Self { rr: r }
//     }
// }
// impl<'a> EnumReservedRange<'a> {
//     /// Inclusive
//     pub fn start(&self) -> i32 {
//         self.rr.start()
//     }
//     /// Inclusive
//     pub fn end(&self) -> i32 {
//         self.rr.end()
//     }
// }
// impl<'a> Copy for EnumReservedRange<'a> {}
// impl<'a> Clone for EnumReservedRange<'a> {
//     fn clone(&self) -> Self {
//         Self { rr: self.rr }
//     }
// }

// #[derive(Debug)]
// pub struct EnumReservedRanges<'a> {
//     ranges: &'a [prost_types::enum_descriptor_proto::EnumReservedRange],
//     u: PhantomData<U>,
// }
// impl<'a> IntoIterator for EnumReservedRanges<'a> {
//     type Item = EnumReservedRange<'a>;
//     type IntoIter = EnumReservedRangeIter<'a>;
//     fn into_iter(self) -> Self::IntoIter {
//         self.ranges.into()
//     }
// }
// impl<'a> EnumReservedRanges<'a> {
//     pub fn iter(&self) -> EnumReservedRangeIter<'a> {
//         self.ranges.into()
//     }
//     pub fn len(&self) -> usize {
//         self.ranges.len()
//     }
//     pub fn is_empty(&self) -> bool {
//         self.ranges.is_empty()
//     }
//     pub fn get(&self, index: usize) -> Option<EnumReservedRange<'a>> {
//         self.ranges.get(index).map(|r| r.into())
//     }
//     pub fn is_range_reserved(&self, min: i32, max: i32) -> bool {
//         self.iter().any(|r| r.start() <= min && r.end() >= max)
//     }
//     pub fn is_in_reserved_range(&self, num: i32) -> bool {
//         self.iter().any(|r| r.start() <= num && r.end() >= num)
//     }
// }
// impl<'a> Copy for EnumReservedRanges<'a> {}
// impl<'a> Clone for EnumReservedRanges<'a> {
//     fn clone(&self) -> Self {
//         Self {
//             ranges: self.ranges,
//         }
//     }
// }
// impl<'a> From<&'a Vec<prost_types::enum_descriptor_proto::EnumReservedRange>>
//     for EnumReservedRanges<'a>
// {
//     fn from(ranges: &'a Vec<prost_types::enum_descriptor_proto::EnumReservedRange>) -> Self {
//         Self { ranges }
//     }
// }
