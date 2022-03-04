use std::{marker::PhantomData, slice};

use lazy_static::lazy_static;

use catalyze::{iter::CommentsIter, WellKnownType};

// use super::{iter::*, *};

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

#[derive(Debug)]
/// Describes a complete .proto file.
pub struct FileDescriptor<'a, U> {
    desc: &'a prost_types::FileDescriptorProto,
    u: PhantomData<U>,
}
impl<'a, U> FileDescriptor<'a, U> {
    /// file name, relative to root of source tree
    pub fn name(&self) -> &'a str {
        self.desc.name()
    }
    /// e.g. "foo", "foo.bar", etc.
    pub fn package(&self) -> &'a str {
        self.desc.package()
    }
    /// Names of files imported by this file.
    pub fn dependencies(&self) -> slice::Iter<String> {
        self.desc.dependency.iter()
    }

    /// Indexes of the public imported files in the dependency list.
    pub fn public_dependencies(&self) -> slice::Iter<i32> {
        self.desc.public_dependency.iter()
    }
    /// All top-level `Message` definitions in this file.
    pub fn messages(&self) -> MessageDescriptorIter<'a, U> {
        (&self.desc.message_type).into()
    }

    pub fn comments(&self) -> CommentsIter<'a, U> {
        self.source_code_info().into()
    }

    // All top-level `Enum` definitions in this file.
    pub fn enums(&self) -> EnumDescriptorIter<'a, U> {
        (&self.desc.enum_type).into()
    }

    /// All top-level `Service` definitions in this file.
    pub fn services(&self) -> ServiceDescriptorIter<'a, U> {
        (&self.desc.service).into()
    }
    pub fn extensions(&self) -> FieldDescriptorIter<'a, U> {
        (&self.desc.extension).into()
    }

    pub fn options(&self) -> FileOptions<'a, U> {
        self.desc.options.as_ref().into()
    }
    /// This field contains optional information about the original source code.
    /// You may safely remove this entire field without harming runtime
    /// functionality of the descriptors -- the information is needed only by
    /// development tools.
    pub fn source_code_info(&self) -> SourceCodeInfo<'a, U> {
        self.desc.source_code_info.as_ref().into()
    }
    /// The syntax of the proto file.
    /// The supported values are "proto2" and "proto3".
    pub fn syntax(&self) -> Syntax {
        self.desc.syntax().into()
    }
}
impl<'a, U> From<&'a prost_types::FileDescriptorProto> for FileDescriptor<'a, U> {
    fn from(desc: &'a prost_types::FileDescriptorProto) -> Self {
        Self {
            desc,
            u: PhantomData,
        }
    }
}
impl<'a, U> Copy for FileDescriptor<'a, U> {}
impl<'a, U> Clone for FileDescriptor<'a, U> {
    fn clone(&self) -> Self {
        Self {
            desc: self.desc,
            u: PhantomData,
        }
    }
}

/// Describes a message type.
#[derive(Debug)]
pub struct MessageDescriptor<'a, U> {
    desc: &'a prost_types::DescriptorProto,
    u: PhantomData<U>,
}
impl<'a, U> MessageDescriptor<'a, U> {
    pub fn name(&self) -> &'a str {
        self.desc.name()
    }
    pub fn fields(&self) -> FieldDescriptorIter<'a, U> {
        let fields = &self.desc.field;
        fields.into()
    }

    pub fn extensions(&self) -> FieldDescriptorIter<'a, U> {
        (&self.desc.extension).into()
    }

    pub fn nested_messages(&self) -> MessageDescriptorIter<'a, U> {
        (&self.desc.nested_type).into()
    }

    pub fn enums(&self) -> EnumDescriptorIter<'a, U> {
        (&self.desc.enum_type).into()
    }

    pub fn extension_ranges(&self) -> ExtensionRanges<'a, U> {
        (&self.desc.extension_range).into()
    }

    pub fn oneofs(&self) -> OneofDescriptorIter<'a, U> {
        (&self.desc.oneof_decl).into()
    }
    pub fn options(&self) -> MessageOptions<'a, U> {
        self.desc.options.as_ref().into()
    }
    pub fn reserved_ranges(&self) -> ReservedRanges<'a, U> {
        (&self.desc.reserved_range).into()
    }
    /// Reserved field names, which may not be used by fields in the same message.
    /// A given name may only be reserved once.
    pub fn reserved_names(&self) -> slice::Iter<String> {
        self.desc.reserved_name.iter()
    }
}
impl<'a, U> From<&'a prost_types::DescriptorProto> for MessageDescriptor<'a, U> {
    fn from(desc: &'a prost_types::DescriptorProto) -> Self {
        Self {
            desc,
            u: PhantomData,
        }
    }
}
impl<'a, U> Copy for MessageDescriptor<'a, U> {}
impl<'a, U> Clone for MessageDescriptor<'a, U> {
    fn clone(&self) -> Self {
        Self {
            desc: self.desc,
            u: PhantomData,
        }
    }
}
#[derive(Debug)]
pub struct EnumDescriptor<'a, U> {
    desc: &'a prost_types::EnumDescriptorProto,
    u: PhantomData<U>,
}
impl<'a, U> EnumDescriptor<'a, U> {
    pub fn name(&self) -> &'a str {
        self.desc.name()
    }
    pub fn values(&self) -> EnumValueDescriptorIter<'a, U> {
        (&self.desc.value).into()
    }
    pub fn options(&self) -> EnumOptions<'a, U> {
        self.desc.options.as_ref().into()
    }
    /// Range of reserved numeric values. Reserved numeric values may not be used
    /// by enum values in the same enum declaration. Reserved ranges may not
    /// overlap.
    pub fn reserved_ranges(&self) -> EnumReservedRanges<'a, U> {
        (&self.desc.reserved_range).into()
    }
    pub fn reserved_names(&self) -> slice::Iter<String> {
        self.desc.reserved_name.iter()
    }
}
impl<'a, U> Copy for EnumDescriptor<'a, U> {}
impl<'a, U> Clone for EnumDescriptor<'a, U> {
    fn clone(&self) -> Self {
        Self {
            desc: self.desc,
            u: PhantomData,
        }
    }
}
impl<'a, U> From<&'a prost_types::EnumDescriptorProto> for EnumDescriptor<'a, U> {
    fn from(desc: &'a prost_types::EnumDescriptorProto) -> Self {
        EnumDescriptor {
            desc,
            u: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct EnumValueDescriptor<'a, U> {
    desc: &'a prost_types::EnumValueDescriptorProto,
    u: PhantomData<U>,
}
impl<'a, U> EnumValueDescriptor<'a, U> {
    pub fn name(&self) -> &'a str {
        self.desc.name()
    }
    pub fn number(&self) -> i32 {
        self.desc.number()
    }
    pub fn options(&self) -> EnumValueOptions<'a, U> {
        self.desc.options.as_ref().into()
    }
}
impl<'a, U> Copy for EnumValueDescriptor<'a, U> {}
impl<'a, U> Clone for EnumValueDescriptor<'a, U> {
    fn clone(&self) -> Self {
        Self {
            desc: self.desc,
            u: PhantomData,
        }
    }
}
impl<'a, U> From<&'a prost_types::EnumValueDescriptorProto> for EnumValueDescriptor<'a, U> {
    fn from(desc: &'a prost_types::EnumValueDescriptorProto) -> Self {
        Self {
            desc,
            u: PhantomData,
        }
    }
}

/// Describes a field within a message.
#[derive(Debug)]
pub struct FieldDescriptor<'a, U> {
    desc: &'a prost_types::FieldDescriptorProto,
    u: PhantomData<U>,
}

impl<'a, U> FieldDescriptor<'a, U> {
    pub fn name(&self) -> &str {
        self.desc.name()
    }
    pub fn number(&self) -> i32 {
        self.desc.number()
    }
    pub fn label(&self) -> Label {
        Label::from(self.desc.label())
    }

    pub fn well_known_type(&self) -> Option<WellKnownType> {
        todo!()
    }

    pub fn is_well_known_type(&self) -> bool {
        self.well_known_type().is_some()
    }

    /// If type_name is set, this need not be set.  If both this and type_name
    /// are set, this must be one of Enum, Message or Group.
    pub fn r#type(&self) -> Type {
        Type::from(self.desc.r#type())
    }

    pub fn is_embed(&self) -> bool {
        matches!(self.r#type(), Type::Message)
    }

    pub fn is_enum(&self) -> bool {
        matches!(self.r#type(), Type::Enum)
    }

    pub fn is_scalar(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(_))
    }

    pub fn is_double(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Double))
    }
    pub fn is_float(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Float))
    }

    pub fn is_int64(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Int64))
    }

    pub fn is_uint64(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Uint64))
    }

    pub fn is_int32(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Int32))
    }

    pub fn is_fixed64(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Fixed64))
    }

    pub fn is_fixed32(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Fixed32))
    }

    pub fn is_bool(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Bool))
    }

    pub fn is_string(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::String))
    }

    pub fn is_bytes(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Bytes))
    }

    pub fn is_uint32(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Uint32))
    }

    pub fn is_sfixed32(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Sfixed32))
    }

    pub fn is_sfixed64(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Sfixed64))
    }

    pub fn is_sint32(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Sint32))
    }

    pub fn is_sint64(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Sint64))
    }
    pub fn is_message(&self) -> bool {
        matches!(self.r#type(), Type::Message)
    }

    pub fn is_repeated(&self) -> bool {
        self.label() == Label::Repeated
    }

    /// alias for `r#type`
    ///
    /// If type_name is set, this need not be set.  If both this and type_name
    /// are set, this must be one of Enum, Message or Group.
    pub fn proto_type(&self) -> Type {
        self.r#type()
    }

    /// For message and enum types, this is the name of the type.  If the name
    /// starts with a '.', it is fully-qualified.  Otherwise, C++-like scoping
    /// rules are used to find the type (i.e. first the nested types within this
    /// message are searched, then within the parent, on up to the root
    /// namespace).
    pub fn type_name(&self) -> &str {
        self.desc.type_name()
    }

    /// For extensions, this is the name of the type being extended.  It is
    /// resolved in the same manner as `proto_type_name`.
    pub fn extendee(&self) -> &str {
        self.desc.extendee()
    }
    /// For numeric types, contains the original text representation of the value.
    /// For booleans, "true" or "false".
    /// For strings, contains the default text contents (not escaped in any way).
    /// For bytes, contains the C escaped value.  All bytes >= 128 are escaped.
    pub fn default_value(&self) -> &str {
        self.desc.default_value()
    }
    /// If set, gives the index of a oneof in the containing type's oneof_decl
    /// list.
    ///
    /// This field is a member of that oneof.
    pub fn oneof_index(&self) -> i32 {
        self.desc.oneof_index()
    }

    /// JSON name of this field. The value is set by protocol compiler. If the
    /// user has set a "json_name" option on this field, that option's value
    /// will be used. Otherwise, it's deduced from the field's name by converting
    /// it to camelCase.
    pub fn json_name(&self) -> &str {
        self.desc.json_name()
    }
    pub fn options(&self) -> FieldOptions<'a, U> {
        self.desc.options.as_ref().into()
    }
    /// If true, this is a proto3 "optional". When a proto3 field is optional, it
    /// tracks presence regardless of field type.
    ///
    /// When proto3_optional is true, this field must be belong to a oneof to
    /// signal to old proto3 clients that presence is tracked for this field. This
    /// oneof is known as a "synthetic" oneof, and this field must be its sole
    /// member (each proto3 optional field gets its own synthetic oneof). Synthetic
    /// oneofs exist in the descriptor only, and do not generate any API. Synthetic
    /// oneofs must be ordered after all "real" oneofs.
    ///
    /// For message fields, proto3_optional doesn't create any semantic change,
    /// since non-repeated message fields always track presence. However it still
    /// indicates the semantic detail of whether the user wrote "optional" or not.
    /// This can be useful for round-tripping the .proto file. For consistency we
    /// give message fields a synthetic oneof also, even though it is not required
    /// to track presence. This is especially important because the parser can't
    /// tell if a field is a message or an enum, so it must always create a
    /// synthetic oneof.
    ///
    /// Proto2 optional fields do not set this flag, because they already indicate
    /// optional with `LABEL_OPTIONAL`.
    pub fn proto3_optional(&self) -> bool {
        self.desc.proto3_optional()
    }
    /// returns `true` if:
    ///
    /// - `syntax` is `Syntax::Proto3` and `proto3_optional` is `true`
    /// - `syntax` is `Syntax::Proto2` and `label` is `Label::Optional`.
    pub fn is_marked_optional(&self, syntax: Syntax) -> bool {
        match syntax {
            Syntax::Proto2 => self.label() == Label::Optional,
            Syntax::Proto3 => self.proto3_optional(),
        }
    }
    pub fn is_required(&self, syntax: Syntax) -> bool {
        syntax.supports_required_prefix() && self.label() == Label::Required
    }
}
impl<'a, U> From<&'a prost_types::FieldDescriptorProto> for FieldDescriptor<'a, U> {
    fn from(desc: &'a prost_types::FieldDescriptorProto) -> Self {
        Self {
            desc,
            u: PhantomData,
        }
    }
}
impl<'a, U> Copy for FieldDescriptor<'a, U> {}
impl<'a, U> Clone for FieldDescriptor<'a, U> {
    fn clone(&self) -> Self {
        Self {
            desc: self.desc,
            u: PhantomData,
        }
    }
}

/// Describes a service.
#[derive(Debug)]
pub struct ServiceDescriptor<'a, U> {
    desc: &'a prost_types::ServiceDescriptorProto,
    u: PhantomData<U>,
}
impl<'a, U> ServiceDescriptor<'a, U> {
    pub fn name(&self) -> &'a str {
        self.desc.name()
    }
    pub fn options(&self) -> ServiceOptions<'a, U> {
        self.desc.options.as_ref().into()
    }
    pub fn methods(&self) -> MethodDescriptorIter<'a, U> {
        (&self.desc.method).into()
    }
}
impl<'a, U> From<&'a prost_types::ServiceDescriptorProto> for ServiceDescriptor<'a, U> {
    fn from(desc: &'a prost_types::ServiceDescriptorProto) -> Self {
        Self {
            desc,
            u: PhantomData,
        }
    }
}
impl<'a, U> Copy for ServiceDescriptor<'a, U> {}
impl<'a, U> Clone for ServiceDescriptor<'a, U> {
    fn clone(&self) -> Self {
        Self {
            desc: self.desc,
            u: PhantomData,
        }
    }
}

/// Describes a method.
#[derive(Debug)]
pub struct MethodDescriptor<'a, U> {
    desc: &'a prost_types::MethodDescriptorProto,
    u: PhantomData<U>,
}
impl<'a, U> MethodDescriptor<'a, U> {
    pub fn name(&self) -> &'a str {
        self.desc.name()
    }
    /// Input type name.
    ///
    /// These are resolved in the same way as
    /// `FieldDescriptor.type_name`, but must refer to a message type
    pub fn input_type(&self) -> &'a str {
        self.desc.input_type()
    }
    /// Output type name.
    ///
    /// These are resolved in the same way as
    /// `FieldDescriptor.type_name`, but must refer to a message type
    pub fn output_type(&self) -> &'a str {
        self.desc.output_type()
    }
    /// Identifies if client streams multiple client messages
    pub fn client_streaming(&self) -> bool {
        self.desc.client_streaming()
    }
    /// Identifies if server streams multiple server messages
    pub fn server_streaming(&self) -> bool {
        self.desc.server_streaming()
    }
    pub fn options(&self) -> MethodOptions<'a, U> {
        self.desc.options.as_ref().into()
    }
}
impl<'a, U> From<&'a prost_types::MethodDescriptorProto> for MethodDescriptor<'a, U> {
    fn from(desc: &'a prost_types::MethodDescriptorProto) -> Self {
        Self {
            desc,
            u: PhantomData,
        }
    }
}
impl<'a, U> Copy for MethodDescriptor<'a, U> {}
impl<'a, U> Clone for MethodDescriptor<'a, U> {
    fn clone(&self) -> Self {
        Self {
            desc: self.desc,
            u: PhantomData,
        }
    }
}

/// Describes a oneof.
#[derive(Debug)]
pub struct OneofDescriptor<'a, U> {
    desc: &'a prost_types::OneofDescriptorProto,
    u: PhantomData<U>,
}
impl<'a, U> OneofDescriptor<'a, U> {
    pub fn name(&self) -> &'a str {
        self.desc.name()
    }
    pub fn options(&self) -> OneofOptions<'a, U> {
        self.desc.options.as_ref().into()
    }
}
impl<'a, U> From<&'a prost_types::OneofDescriptorProto> for OneofDescriptor<'a, U> {
    fn from(desc: &'a prost_types::OneofDescriptorProto) -> Self {
        Self {
            desc,
            u: PhantomData,
        }
    }
}
impl<'a, U> Copy for OneofDescriptor<'a, U> {}
impl<'a, U> Clone for OneofDescriptor<'a, U> {
    fn clone(&self) -> Self {
        Self {
            desc: self.desc,
            u: PhantomData,
        }
    }
}

// ===================================================================
// Options

// Each of the definitions above may have "options" attached.  These are
// just annotations which may cause code to be generated slightly differently
// or may contain hints for code that manipulates protocol messages.
//
// Clients may define custom options as extensions of the *Options messages.
// These extensions may not yet be known at parsing time, so the parser cannot
// store the values in them.  Instead it stores them in a field in the *Options
// message called uninterpreted_option. This field must have the same name
// across all *Options messages. We then use this field to populate the
// extensions when we build a descriptor, at which point all protos have been
// parsed and so all extensions are known.
//
// Extension numbers for custom options may be chosen as follows:
// * For options which will only be used within a single application or
//   organization, or for experimental options, use field numbers 50000
//   through 99999.  It is up to you to ensure that you do not use the
//   same number for multiple options.
// * For options which will be published and used publicly by multiple
//   independent entities, e-mail protobuf-global-extension-registry@google.com
//   to reserve extension numbers. Simply provide your project name (e.g.
//   Objective-C plugin) and your project website (if available) -- there's no
//   need to explain how you intend to use them. Usually you only need one
//   extension number. You can declare multiple options with only one extension
//   number by putting them in a sub-message. See the Custom Options section of
//   the docs for examples:
//   <https://developers.google.com/protocol-buffers/docs/proto#options>
//   If this turns out to be popular, a web service will be set up
//   to automatically assign option numbers.

#[derive(Debug)]
pub struct FileOptions<'a, U> {
    opts: Option<&'a prost_types::FileOptions>,
    u: PhantomData<U>,
}
impl<'a, U> From<Option<&'a prost_types::FileOptions>> for FileOptions<'a, U> {
    fn from(opts: Option<&'a prost_types::FileOptions>) -> Self {
        Self {
            opts,
            u: PhantomData,
        }
    }
}
impl<'a, U> FileOptions<'a, U> {
    /// Java package where classes generated from this .proto will be
    /// placed.  By default, the proto package is used, but this is often
    /// inappropriate because proto packages do not normally start with backwards
    /// domain names.
    pub fn java_package(&self) -> &str {
        self.opts().java_package()
    }
    /// If set, all the classes from the .proto file are wrapped in a single
    /// outer class with the given name.  This applies to both Proto1
    /// (equivalent to the old "--one_java_file" option) and Proto2 (where
    /// a .proto always translates to a single class, but you may want to
    /// explicitly choose the class name).
    pub fn java_outer_classname(&self) -> &str {
        self.opts().java_outer_classname()
    }

    /// If set true, then the Java code generator will generate a separate .java
    /// file for each top-level message, enum, and service defined in the .proto
    /// file.  Thus, these types will *not* be nested inside the outer class
    /// named by java_outer_classname.  However, the outer class will still be
    /// generated to contain the file's getDescriptor() method as well as any
    /// top-level extensions defined in the file.
    pub fn java_multiple_files(&self) -> bool {
        self.opts().java_multiple_files()
    }

    /// If set true, then the Java2 code generator will generate code that
    /// throws an exception whenever an attempt is made to assign a non-UTF-8
    /// byte sequence to a string field.
    /// Message reflection will do the same.
    /// However, an extension field still accepts non-UTF-8 byte sequences.
    /// This option has no effect on when used with the lite runtime.    
    pub fn java_string_check_utf8(&self) -> bool {
        self.opts().java_string_check_utf8()
    }
    /// Generated classes can be optimized for speed or code size.
    pub fn optimize_for(&self) -> OptimizeMode {
        self.opts().optimize_for().into()
    }
    /// Sets the Go package where structs generated from this .proto will be
    /// placed. If omitted, the Go package will be derived from the following:
    ///   - The basename of the package import path, if provided.
    ///   - Otherwise, the package statement in the .proto file, if present.
    ///   - Otherwise, the basename of the .proto file, without extension.
    pub fn go_package(&self) -> &str {
        self.opts().go_package()
    }
    /// Should generic services be generated in each language?  "Generic" services
    /// are not specific to any particular RPC system.  They are generated by the
    /// main code generators in each language (without additional plugins).
    /// Generic services were the only kind of service generation supported by
    /// early versions of google.protobuf.
    ///
    /// Generic services are now considered deprecated in favor of using plugins
    /// that generate code specific to your particular RPC system.  Therefore,
    /// these default to false.  Old code which depends on generic services should
    /// explicitly set them to true.
    pub fn cc_generic_services(&self) -> bool {
        self.opts().cc_generic_services()
    }

    pub fn java_generic_services(&self) -> bool {
        self.opts().java_generic_services()
    }

    pub fn py_generic_services(&self) -> bool {
        self.opts().py_generic_services()
    }

    pub fn php_generic_services(&self) -> bool {
        self.opts().php_generic_services()
    }

    /// Is this file deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for everything in the file, or it will be completely ignored; in the very
    /// least, this is a formalization for deprecating files.
    pub fn deprecated(&self) -> bool {
        self.opts().deprecated()
    }
    pub fn is_deprecated(&self) -> bool {
        self.opts().deprecated()
    }
    /// Enables the use of arenas for the proto messages in this file. This applies
    /// only to generated classes for C++.
    pub fn cc_enable_arenas(&self) -> bool {
        self.opts().cc_enable_arenas()
    }
    /// Sets the objective c class prefix which is prepended to all objective c
    /// generated classes from this .proto. There is no default.
    pub fn objc_class_prefix(&self) -> &str {
        self.opts().objc_class_prefix()
    }

    /// Namespace for generated classes; defaults to the package.    
    pub fn csharp_namespace(&self) -> &str {
        self.opts().csharp_namespace()
    }
    /// By default Swift generators will take the proto package and CamelCase it
    /// replacing '.' with underscore and use that to prefix the types/symbols
    /// defined. When this options is provided, they will use this value instead
    /// to prefix the types/symbols defined.
    pub fn swift_prefix(&self) -> &str {
        self.opts().swift_prefix()
    }

    /// Sets the php class prefix which is prepended to all php generated classes
    /// from this .proto. Default is empty.
    pub fn php_class_prefix(&self) -> &str {
        self.opts().php_class_prefix()
    }

    /// Use this option to change the namespace of php generated classes. Default
    /// is empty. When this option is empty, the package name will be used for
    /// determining the namespace.
    pub fn php_namespace(&self) -> &str {
        self.opts().php_namespace()
    }

    /// Use this option to change the namespace of php generated metadata classes.
    /// Default is empty. When this option is empty, the proto file name will be
    /// used for determining the namespace.
    pub fn php_metadata_namespace(&self) -> &str {
        self.opts().php_metadata_namespace()
    }
    /// Use this option to change the package of ruby generated classes. Default
    /// is empty. When this option is not set, the package name will be used for
    /// determining the ruby package.
    pub fn ruby_package(&self) -> &str {
        self.opts().ruby_package()
    }
    /// The parser stores options it doesn't recognize here.
    /// See the documentation for the "Options" section above.
    pub fn uninterpreted_options(&self) -> UninterpretedOptions<'a, U> {
        (&self.opts().uninterpreted_option).into()
    }
    fn opts(&self) -> &'a prost_types::FileOptions {
        self.opts.unwrap_or(&DEFAULT_FILE_OPTIONS)
    }
}
impl<'a, U> Copy for FileOptions<'a, U> {}
impl<'a, U> Clone for FileOptions<'a, U> {
    fn clone(&self) -> Self {
        FileOptions {
            opts: self.opts,
            u: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct EnumValueOptions<'a, U> {
    opts: Option<&'a prost_types::EnumValueOptions>,
    u: PhantomData<&'a U>,
}
impl<'a, U> EnumValueOptions<'a, U> {
    /// Is this enum value deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the enum value, or it will be completely ignored; in the very least,
    /// this is a formalization for deprecating enum values.
    pub fn deprecated(&self) -> bool {
        self.opts().deprecated()
    }
    pub fn is_deprecated(&self) -> bool {
        self.opts().deprecated()
    }
    /// Options not recognized by the parser.
    pub fn uninterpreted_options(&self) -> UninterpretedOptions<'a, U> {
        (&self.opts().uninterpreted_option).into()
    }
    fn opts(&self) -> &'a prost_types::EnumValueOptions {
        self.opts.unwrap_or(&DEFAULT_ENUM_VALUE_OPTIONS)
    }
}
impl<'a, U> From<Option<&'a prost_types::EnumValueOptions>> for EnumValueOptions<'a, U> {
    fn from(opts: Option<&'a prost_types::EnumValueOptions>) -> Self {
        Self {
            opts,
            u: PhantomData,
        }
    }
}
impl<'a, U> Copy for EnumValueOptions<'a, U> {}
impl<'a, U> Clone for EnumValueOptions<'a, U> {
    fn clone(&self) -> Self {
        Self {
            opts: self.opts,
            u: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct MessageOptions<'a, U> {
    opts: Option<&'a prost_types::MessageOptions>,
    u: PhantomData<U>,
}
impl<'a, U> MessageOptions<'a, U> {
    /// Set true to use the old proto1 MessageSet wire format for extensions.
    /// This is provided for backwards-compatibility with the MessageSet wire
    /// format.  You should not use this for any other reason:  It's less
    /// efficient, has fewer features, and is more complicated.
    ///
    /// The message must be defined exactly as follows:
    ///   message Foo {
    ///     option message_set_wire_format = true;
    ///     extensions 4 to max;
    ///   }
    /// Note that the message cannot have any defined fields; MessageSets only
    /// have extensions.
    ///
    /// All extensions of your type must be singular messages; e.g. they cannot
    /// be int32s, enums, or repeated messages.
    ///
    /// Because this is an option, the above two restrictions are not enforced by
    /// the protocol compiler.
    pub fn message_set_wire_format(&self) -> bool {
        self.opts().message_set_wire_format()
    }
    /// Whether the message is an automatically generated map entry type for the
    /// maps field.
    ///
    /// For maps fields:
    ///     map<KeyType, ValueType> map_field = 1;
    /// The parsed descriptor looks like:
    ///     message MapFieldEntry {
    ///         option map_entry = true;
    ///         optional KeyType key = 1;
    ///         optional ValueType value = 2;
    ///     }
    ///     repeated MapFieldEntry map_field = 1;
    ///
    /// Implementations may choose not to generate the map_entry=true message, but
    /// use a native map in the target language to hold the keys and values.
    /// The reflection APIs in such implementations still need to work as
    /// if the field is a repeated message field.
    ///
    /// NOTE: Do not set the option in .proto files. Always use the maps syntax
    /// instead. The option should only be implicitly set by the proto compiler
    /// parser.
    pub fn map_entry(&self) -> bool {
        self.opts().map_entry()
    }

    pub fn is_map_entry(&self) -> bool {
        self.map_entry()
    }

    pub fn deprecated(&self) -> bool {
        self.opts().deprecated()
    }
    pub fn is_deprecated(&self) -> bool {
        self.opts().deprecated()
    }
    pub fn no_standard_descriptor_accessor(&self) -> bool {
        self.opts().no_standard_descriptor_accessor()
    }
    /// The parser stores options it doesn't recognize here. See above.
    pub fn uninterpreted_option(&self) -> UninterpretedOptions<'a, U> {
        (&self.opts().uninterpreted_option).into()
    }
    fn opts(&self) -> &'a prost_types::MessageOptions {
        self.opts.unwrap_or(&DEFAULT_MESSAGE_OPTIONS)
    }
}
impl<'a, U> From<Option<&'a prost_types::MessageOptions>> for MessageOptions<'a, U> {
    fn from(opts: Option<&'a prost_types::MessageOptions>) -> Self {
        MessageOptions {
            opts,
            u: PhantomData,
        }
    }
}
impl<'a, U> Copy for MessageOptions<'a, U> {}
impl<'a, U> Clone for MessageOptions<'a, U> {
    fn clone(&self) -> Self {
        MessageOptions {
            opts: self.opts,
            u: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct FieldOptions<'a, U> {
    opts: Option<&'a prost_types::FieldOptions>,
    u: PhantomData<U>,
}
impl<'a, U> FieldOptions<'a, U> {
    pub fn new(opts: Option<&'a prost_types::FieldOptions>) -> Self {
        Self {
            opts,
            u: PhantomData,
        }
    }
    /// The ctype option instructs the C++ code generator to use a different
    /// representation of the field than it normally would.  See the specific
    /// options below.  This option is not yet implemented in the open source
    /// release -- sorry, we'll try to include it in a future version!
    pub fn c_type(&self) -> CType {
        CType::from(self.opts().ctype())
    }
    /// The packed option can be enabled for repeated primitive fields to enable
    /// a more efficient representation on the wire. Rather than repeatedly
    /// writing the tag and type for each element, the entire array is encoded as
    /// a single length-delimited blob. In proto3, only explicit setting it to
    /// false will avoid using packed encoding.
    pub fn packed(&self) -> bool {
        self.opts().packed()
    }
    /// The jstype option determines the JavaScript type used for values of the
    /// field.  The option is permitted only for 64 bit integral and fixed types
    /// (int64, uint64, sint64, fixed64, sfixed64).  A field with jstype JS_STRING
    /// is represented as JavaScript string, which avoids loss of precision that
    /// can happen when a large value is converted to a floating point JavaScript.
    /// Specifying JS_NUMBER for the jstype causes the generated JavaScript code to
    /// use the JavaScript "number" type.  The behavior of the default option
    /// JS_NORMAL is implementation dependent.
    ///
    /// This option is an enum to permit additional types to be added, e.g.
    /// goog.math.Integer.
    pub fn js_type(&self) -> JsType {
        self.opts().jstype().into()
    }
    /// Should this field be parsed lazily?  Lazy applies only to message-type
    /// fields.  It means that when the outer message is initially parsed, the
    /// inner message's contents will not be parsed but instead stored in encoded
    /// form.  The inner message will actually be parsed when it is first accessed.
    ///
    /// This is only a hint.  Implementations are free to choose whether to use
    /// eager or lazy parsing regardless of the value of this option.  However,
    /// setting this option true suggests that the protocol author believes that
    /// using lazy parsing on this field is worth the additional bookkeeping
    /// overhead typically needed to implement it.
    ///
    /// This option does not affect the public interface of any generated code;
    /// all method signatures remain the same.  Furthermore, thread-safety of the
    /// interface is not affected by this option; const methods remain safe to
    /// call from multiple threads concurrently, while non-const methods continue
    /// to require exclusive access.
    ///
    ///
    /// Note that implementations may choose not to check required fields within
    /// a lazy sub-message.  That is, calling IsInitialized() on the outer message
    /// may return true even if the inner message has missing required fields.
    /// This is necessary because otherwise the inner message would have to be
    /// parsed in order to perform the check, defeating the purpose of lazy
    /// parsing.  An implementation which chooses not to check required fields
    /// must be consistent about it.  That is, for any particular sub-message, the
    /// implementation must either *always* check its required fields, or *never*
    /// check its required fields, regardless of whether or not the message has
    /// been parsed.
    pub fn is_lazy(&self) -> bool {
        self.opts().lazy()
    }
    /// Is this field deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for accessors, or it will be completely ignored; in the very least, this
    /// is a formalization for deprecating fields.
    pub fn deprecated(&self) -> bool {
        self.opts().deprecated()
    }
    pub fn is_deprecated(&self) -> bool {
        self.opts().deprecated()
    }
    /// For Google-internal migration only. Do not use.
    pub fn is_weak(&self) -> bool {
        self.opts().weak()
    }

    /// Options the parser does not recognize.
    pub fn uninterpreted_options(&self) -> UninterpretedOptions<'a, U> {
        (&self.opts().uninterpreted_option).into()
    }

    fn opts(&self) -> &'a prost_types::FieldOptions {
        self.opts.unwrap_or(&DEFAULT_FIELD_OPTIONS)
    }
}
impl<'a, U> From<Option<&'a prost_types::FieldOptions>> for FieldOptions<'a, U> {
    fn from(opts: Option<&'a prost_types::FieldOptions>) -> Self {
        Self {
            opts,
            u: PhantomData,
        }
    }
}
impl<'a, U> Copy for FieldOptions<'a, U> {}
impl<'a, U> Clone for FieldOptions<'a, U> {
    fn clone(&self) -> Self {
        Self {
            opts: self.opts,
            u: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct EnumOptions<'a, U> {
    opts: Option<&'a prost_types::EnumOptions>,
    u: PhantomData<U>,
}
impl<'a, U> EnumOptions<'a, U> {
    /// Is this enum deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the enum, or it will be completely ignored; in the very least, this
    /// is a formalization for deprecating enums.
    pub fn deprecated(&self) -> bool {
        self.opts().deprecated()
    }
    pub fn is_deprecated(&self) -> bool {
        self.opts().deprecated()
    }
    /// Options not recognized by the parser.
    pub fn uninterpreted_options(&self) -> UninterpretedOptions<'a, U> {
        (&self.opts().uninterpreted_option).into()
    }
    /// Allows mapping different tag names to the same value.
    pub fn allow_alias(&self) -> bool {
        self.opts().allow_alias()
    }
    fn opts(&self) -> &'a prost_types::EnumOptions {
        self.opts.unwrap_or(&DEFAULT_ENUM_OPTIONS)
    }
}
impl<'a, U> From<Option<&'a prost_types::EnumOptions>> for EnumOptions<'a, U> {
    fn from(opts: Option<&'a prost_types::EnumOptions>) -> Self {
        Self {
            opts,
            u: PhantomData,
        }
    }
}
impl<'a, U> Copy for EnumOptions<'a, U> {}
impl<'a, U> Clone for EnumOptions<'a, U> {
    fn clone(&self) -> Self {
        Self {
            opts: self.opts,
            u: PhantomData,
        }
    }
}

/// Options for a Service.
///
/// Note: Field numbers 1 through 32 are reserved for Google's internal RPC
/// framework.
#[derive(Debug)]
pub struct ServiceOptions<'a, U> {
    opts: Option<&'a prost_types::ServiceOptions>,
    u: PhantomData<U>,
}
impl<'a, U> ServiceOptions<'a, U> {
    /// Is this service deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the service, or it will be completely ignored; in the very least,
    /// this is a formalization for deprecating services.
    pub fn deprecated(&self) -> bool {
        self.opts().deprecated()
    }
    pub fn is_deprecated(&self) -> bool {
        self.opts().deprecated()
    }
    /// The parser stores options it doesn't recognize here. See above.
    pub fn uninterpreted_options(&self) -> UninterpretedOptions<'a, U> {
        (&self.opts().uninterpreted_option).into()
    }
    fn opts(&self) -> &'a prost_types::ServiceOptions {
        self.opts.unwrap_or(&DEFAULT_SERVICE_OPTIONS)
    }
}
impl<'a, U> From<Option<&'a prost_types::ServiceOptions>> for ServiceOptions<'a, U> {
    fn from(opts: Option<&'a prost_types::ServiceOptions>) -> Self {
        Self {
            opts,
            u: PhantomData,
        }
    }
}
impl<'a, U> Copy for ServiceOptions<'a, U> {}
impl<'a, U> Clone for ServiceOptions<'a, U> {
    fn clone(&self) -> Self {
        Self {
            opts: self.opts,
            u: PhantomData,
        }
    }
}

/// Options for a Method.
///
/// Note:  Field numbers 1 through 32 are reserved for Google's internal RPC
/// framework.
pub struct MethodOptions<'a, U> {
    opts: Option<&'a prost_types::MethodOptions>,
    u: PhantomData<U>,
}
impl<'a, U> MethodOptions<'a, U> {
    // Note:  Field numbers 1 through 32 are reserved for Google's internal RPC
    //   framework.  We apologize for hoarding these numbers to ourselves, but
    //   we were already using them long before we decided to release Protocol
    //   Buffers.

    /// Is this method deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the method, or it will be completely ignored; in the very least,
    /// this is a formalization for deprecating methods.
    pub fn deprecated(&self) -> bool {
        self.opts().deprecated()
    }
    pub fn is_deprecated(&self) -> bool {
        self.opts().deprecated()
    }
    /// The parser stores options it doesn't recognize here. See above.
    pub fn uninterpreted_options(&self) -> UninterpretedOptions<'a, U> {
        (&self.opts().uninterpreted_option).into()
    }

    /// Is this method side-effect-free (or safe in HTTP parlance), or idempotent,
    /// or neither? HTTP based RPC implementation may choose GET verb for safe
    /// methods, and PUT verb for idempotent methods instead of the default POST.
    pub fn idempotency_level(&self) -> IdempotencyLevel {
        self.opts().idempotency_level().into()
    }
    fn opts(&self) -> &'a prost_types::MethodOptions {
        self.opts.unwrap_or(&DEFAULT_METHOD_OPTIONS)
    }
}
impl<'a, U> From<Option<&'a prost_types::MethodOptions>> for MethodOptions<'a, U> {
    fn from(opts: Option<&'a prost_types::MethodOptions>) -> Self {
        Self {
            opts,
            u: PhantomData,
        }
    }
}
impl<'a, U> Copy for MethodOptions<'a, U> {}
impl<'a, U> Clone for MethodOptions<'a, U> {
    fn clone(&self) -> Self {
        Self {
            opts: self.opts,
            u: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct OneofOptions<'a, U> {
    opts: Option<&'a prost_types::OneofOptions>,
    u: PhantomData<U>,
}
impl<'a, U> OneofOptions<'a, U> {
    /// The parser stores options it doesn't recognize here. See above.
    pub fn uninterpreted_options(&self) -> UninterpretedOptions<'a, U> {
        (&self.opts().uninterpreted_option).into()
    }
    pub fn opts(&self) -> &'a prost_types::OneofOptions {
        self.opts.unwrap_or(&DEFAULT_ONEOF_OPTIONS)
    }
}
impl<'a, U> From<Option<&'a prost_types::OneofOptions>> for OneofOptions<'a, U> {
    fn from(opts: Option<&'a prost_types::OneofOptions>) -> Self {
        Self {
            opts,
            u: PhantomData,
        }
    }
}
impl<'a, U> Copy for OneofOptions<'a, U> {}
impl<'a, U> Clone for OneofOptions<'a, U> {
    fn clone(&self) -> Self {
        Self {
            opts: self.opts,
            u: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct UninterpretedOptions<'a, U> {
    pub(crate) opts: &'a [prost_types::UninterpretedOption],
    u: PhantomData<U>,
}
impl<'a, U> UninterpretedOptions<'a, U> {
    pub fn iter(&self) -> UninterpretedOptionsIter<'a, U> {
        self.into()
    }
}
impl<'a, U> IntoIterator for UninterpretedOptions<'a, U> {
    type Item = UninterpretedOption<'a, U>;
    type IntoIter = UninterpretedOptionsIter<'a, U>;

    fn into_iter(self) -> Self::IntoIter {
        self.into()
    }
}
impl<'a, U> From<&'a Vec<prost_types::UninterpretedOption>> for UninterpretedOptions<'a, U> {
    fn from(opts: &'a Vec<prost_types::UninterpretedOption>) -> Self {
        UninterpretedOptions {
            opts,
            u: PhantomData,
        }
    }
}
impl<'a, U> Copy for UninterpretedOptions<'a, U> {}
impl<'a, U> Clone for UninterpretedOptions<'a, U> {
    fn clone(&self) -> Self {
        UninterpretedOptions {
            opts: self.opts,
            u: PhantomData,
        }
    }
}

/// A message representing a option the parser does not recognize. This only
/// appears in options protos created by the compiler::Parser class.
/// DescriptorPool resolves these when building Descriptor objects. Therefore,
/// options protos in descriptor objects (e.g. returned by Descriptor::options(),
/// or produced by Descriptor::CopyTo()) will never have UninterpretedOptions
/// in them.
#[derive(Debug)]
pub struct UninterpretedOption<'a, U> {
    opt: &'a prost_types::UninterpretedOption,
    u: PhantomData<U>,
}
impl<'a, U> UninterpretedOption<'a, U> {
    pub fn name_parts(&self) -> NameParts<'a, U> {
        NameParts::from(&self.opt.name)
    }
}
impl<'a, U> From<&'a prost_types::UninterpretedOption> for UninterpretedOption<'a, U> {
    fn from(opt: &'a prost_types::UninterpretedOption) -> Self {
        UninterpretedOption {
            opt,
            u: PhantomData,
        }
    }
}
impl<'a, U> Copy for UninterpretedOption<'a, U> {}
impl<'a, U> Clone for UninterpretedOption<'a, U> {
    fn clone(&self) -> Self {
        UninterpretedOption {
            opt: self.opt,
            u: PhantomData,
        }
    }
}

/// Range of reserved tag numbers. Reserved tag numbers may not be used by
/// fields or extension ranges in the same message. Reserved ranges may
/// not overlap.
#[derive(Debug)]
pub struct ReservedRange<'a, U> {
    range: &'a prost_types::descriptor_proto::ReservedRange,
    u: PhantomData<U>,
}
impl<'a, U> ReservedRange<'a, U> {
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
impl<'a, U> PartialEq for ReservedRange<'a, U> {
    fn eq(&self, other: &Self) -> bool {
        self.start() == other.start() && self.end() == other.end()
    }
}
impl<'a, U> From<&'a prost_types::descriptor_proto::ReservedRange> for ReservedRange<'a, U> {
    fn from(range: &'a prost_types::descriptor_proto::ReservedRange) -> Self {
        ReservedRange {
            range,
            u: PhantomData,
        }
    }
}
impl<'a, U> Copy for ReservedRange<'a, U> {}
impl<'a, U> Clone for ReservedRange<'a, U> {
    fn clone(&self) -> Self {
        ReservedRange {
            range: self.range,
            u: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct ReservedRanges<'a, U> {
    ranges: &'a [prost_types::descriptor_proto::ReservedRange],
    u: PhantomData<U>,
}
impl<'a, U> IntoIterator for ReservedRanges<'a, U> {
    type Item = ReservedRange<'a, U>;
    type IntoIter = ReservedRangeIter<'a, U>;
    fn into_iter(self) -> Self::IntoIter {
        self.ranges.into()
    }
}
impl<'a, U> ReservedRanges<'a, U> {
    pub fn iter(&self) -> ReservedRangeIter<'a, U> {
        self.ranges.into()
    }
    pub fn len(&self) -> usize {
        self.ranges.len()
    }
    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }
    pub fn get(&self, index: usize) -> Option<ReservedRange<'a, U>> {
        self.ranges.get(index).map(Into::into)
    }
    pub fn is_range_reserved(&self, min: i32, max: i32) -> bool {
        self.iter().any(|r| r.start() <= min && r.end() >= max)
    }
    pub fn is_in_reserved_range(&self, num: i32) -> bool {
        self.iter().any(|r| r.start() <= num && r.end() >= num)
    }
}
impl<'a, U> From<&'a Vec<prost_types::descriptor_proto::ReservedRange>> for ReservedRanges<'a, U> {
    fn from(ranges: &'a Vec<prost_types::descriptor_proto::ReservedRange>) -> Self {
        ReservedRanges {
            ranges,
            u: PhantomData,
        }
    }
}
impl<'a, U> Copy for ReservedRanges<'a, U> {}
impl<'a, U> Clone for ReservedRanges<'a, U> {
    fn clone(&self) -> Self {
        ReservedRanges {
            ranges: self.ranges,
            u: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct ExtensionRange<'a, U> {
    range: &'a prost_types::descriptor_proto::ExtensionRange,
    u: PhantomData<U>,
}
impl<'a, U> PartialEq for ExtensionRange<'a, U> {
    fn eq(&self, other: &Self) -> bool {
        self.range.start() == other.start() && self.end() == other.end()
    }
}
impl<'a, U> ExtensionRange<'a, U> {
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
impl<'a, U> From<&'a prost_types::descriptor_proto::ExtensionRange> for ExtensionRange<'a, U> {
    fn from(range: &'a prost_types::descriptor_proto::ExtensionRange) -> Self {
        ExtensionRange {
            range,
            u: PhantomData,
        }
    }
}
impl<'a, U> Copy for ExtensionRange<'a, U> {}
impl<'a, U> Clone for ExtensionRange<'a, U> {
    fn clone(&self) -> Self {
        ExtensionRange {
            range: self.range,
            u: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct ExtensionRanges<'a, U> {
    ranges: &'a [prost_types::descriptor_proto::ExtensionRange],
    u: PhantomData<U>,
}
impl<'a, U> ExtensionRanges<'a, U> {
    pub fn iter(&self) -> ExtensionRangeIter<'a, U> {
        self.ranges.into()
    }
    pub fn len(&self) -> usize {
        self.ranges.len()
    }
    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }
    pub fn first(&self) -> Option<ExtensionRange<'a, U>> {
        self.ranges.first().map(|r| r.into())
    }
    pub fn last(&self) -> Option<ExtensionRange<'a, U>> {
        self.ranges.last().map(|r| r.into())
    }
    pub fn get(&self, n: usize) -> Option<ExtensionRange<'a, U>> {
        self.ranges.get(n).map(|r| r.into())
    }
}
impl<'a, U> IntoIterator for ExtensionRanges<'a, U> {
    type Item = ExtensionRange<'a, U>;
    type IntoIter = ExtensionRangeIter<'a, U>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'a, U> From<&'a Vec<prost_types::descriptor_proto::ExtensionRange>>
    for ExtensionRanges<'a, U>
{
    fn from(ranges: &'a Vec<prost_types::descriptor_proto::ExtensionRange>) -> Self {
        ExtensionRanges {
            ranges,
            u: PhantomData,
        }
    }
}
impl<'a, U> Copy for ExtensionRanges<'a, U> {}
impl<'a, U> Clone for ExtensionRanges<'a, U> {
    fn clone(&self) -> Self {
        ExtensionRanges {
            ranges: self.ranges,
            u: PhantomData,
        }
    }
}

/// Range of reserved numeric values. Reserved values may not be used by
/// entries in the same enum. Reserved ranges may not overlap.
///
/// Note that this is distinct from DescriptorProto.ReservedRange in that it
/// is inclusive such that it can appropriately represent the entire int32
/// domain.
#[derive(Debug, PartialEq)]
pub struct EnumReservedRange<'a, U> {
    u: PhantomData<U>,
    rr: &'a prost_types::enum_descriptor_proto::EnumReservedRange,
}
impl<'a, U> From<&'a prost_types::enum_descriptor_proto::EnumReservedRange>
    for EnumReservedRange<'a, U>
{
    fn from(r: &'a prost_types::enum_descriptor_proto::EnumReservedRange) -> Self {
        Self {
            u: PhantomData,
            rr: r,
        }
    }
}
impl<'a, U> EnumReservedRange<'a, U> {
    /// Inclusive
    pub fn start(&self) -> i32 {
        self.rr.start()
    }
    /// Inclusive
    pub fn end(&self) -> i32 {
        self.rr.end()
    }
}
impl<'a, U> Copy for EnumReservedRange<'a, U> {}
impl<'a, U> Clone for EnumReservedRange<'a, U> {
    fn clone(&self) -> Self {
        Self {
            u: PhantomData,
            rr: self.rr,
        }
    }
}

#[derive(Debug)]
pub struct EnumReservedRanges<'a, U> {
    ranges: &'a [prost_types::enum_descriptor_proto::EnumReservedRange],
    u: PhantomData<U>,
}
impl<'a, U> IntoIterator for EnumReservedRanges<'a, U> {
    type Item = EnumReservedRange<'a, U>;
    type IntoIter = EnumReservedRangeIter<'a, U>;
    fn into_iter(self) -> Self::IntoIter {
        self.ranges.into()
    }
}
impl<'a, U> EnumReservedRanges<'a, U> {
    pub fn iter(&self) -> EnumReservedRangeIter<'a, U> {
        self.ranges.into()
    }
    pub fn len(&self) -> usize {
        self.ranges.len()
    }
    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }
    pub fn get(&self, index: usize) -> Option<EnumReservedRange<'a, U>> {
        self.ranges.get(index).map(|r| r.into())
    }
    pub fn is_range_reserved(&self, min: i32, max: i32) -> bool {
        self.iter().any(|r| r.start() <= min && r.end() >= max)
    }
    pub fn is_in_reserved_range(&self, num: i32) -> bool {
        self.iter().any(|r| r.start() <= num && r.end() >= num)
    }
}
impl<'a, U> Copy for EnumReservedRanges<'a, U> {}
impl<'a, U> Clone for EnumReservedRanges<'a, U> {
    fn clone(&self) -> Self {
        Self {
            ranges: self.ranges,
            u: PhantomData,
        }
    }
}
impl<'a, U> From<&'a Vec<prost_types::enum_descriptor_proto::EnumReservedRange>>
    for EnumReservedRanges<'a, U>
{
    fn from(ranges: &'a Vec<prost_types::enum_descriptor_proto::EnumReservedRange>) -> Self {
        Self {
            ranges,
            u: PhantomData,
        }
    }
}
