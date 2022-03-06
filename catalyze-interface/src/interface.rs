use super::{enums::*, Impl};

use core::fmt::Debug;
use std::{fmt::Display, ops::Deref, slice};

/// Describes a complete .proto file.
pub trait FileDescriptor<'a, I: Impl<'a>>: Clone + Copy + Debug {
    /// file name, relative to root of source tree
    fn name(&self) -> &'a str;
    /// e.g. "foo", "foo.bar", etc.
    fn package(&self) -> &'a str;
    /// Names of files imported by this file.
    fn dependencies(&self) -> slice::Iter<String>;
    /// Indexes of the public imported files in the dependency list.
    fn public_dependencies(&self) -> slice::Iter<i32>;
    /// All top-level `Message` definitions in this file.
    fn messages(&self) -> I::MessageDescriptorIter;
    /// All top-level `Enum` definitions in this file.
    fn enums(&self) -> I::EnumDescriptorIter;
    /// All top-level `Service` definitions in this file.
    fn services(&self) -> I::ServiceDescriptorIter;
    fn options(&self) -> I::FileOptions;
    fn source_code_info(&self) -> I::SourceCodeInfo;
    /// Syntax of this file
    fn syntax(&self) -> Syntax;
}

/// Describes a message type.

pub trait MessageDescriptor<'a, I: Impl<'a>>: Clone + Copy + Debug {
    /// The name of this message type.
    fn name(&self) -> &'a str;
    /// The `Message`'s `Field`s.
    fn fields(&self) -> I::FieldDescriptorIter;
    /// `Extension`s defined in this `Message`.
    fn extensions(&self) -> I::FieldDescriptorIter;
    /// Nested `Message`s defined in this `Message`.
    fn nested_messages(&self) -> I::MessageDescriptor;
    /// Nested `Enum`s defined in this `Message`.
    fn enums(&self) -> I::EnumDescriptor;
    /// Exntension set aside for this `Message`.
    fn extension_ranges(&self) -> I::ExtensionRangeIter;
    /// `Oneof`s defined in this `Message`.
    fn oneofs(&self) -> I::OneofDescriptor;
    fn options(&self) -> I::MessageOptions;
    fn reserved_ranges(&self) -> I::ReservedRanges;
    fn reserved_names(&self) -> slice::Iter<String>;
}

// #[cfg_attr(test, automock)]
pub trait EnumDescriptor<'a, I: Impl<'a>>: Clone + Copy + Debug {
    fn name(&self) -> &'a str;
    fn values(&self) -> I::EnumValueDescriptorIter;
    fn options(&self) -> I::EnumOptions;
    /// Range of reserved numeric values. Reserved numeric values may not be used
    /// by enum values in the same enum declaration. Reserved ranges may not
    /// overlap.   
    fn reserved_ranges(&self) -> I::EnumReservedRanges;
    fn reserved_names(&self) -> slice::Iter<String>;
}

// #[cfg_attr(test, automock)]
pub trait EnumValueDescriptor<'a, I: Impl<'a>>: Clone + Copy + Debug {
    fn name(&self) -> String;
    fn number(&self) -> i32;
    fn options(&self) -> I::EnumValueOptions;
}

/// Describes a field within a message.
// #[cfg_attr(test, automock)]
pub trait FieldDescriptor<'a, I: Impl<'a>>: Clone + Copy + Debug {
    fn name(&self) -> String;
    fn number(&self) -> i32;
    fn label(&self) -> Label;
    fn well_known_type(&self) -> Option<WellKnownType>;

    /// If type_name is set, this need not be set.  If both this and type_name
    /// are set, this must be one of Enum, Message or Group.
    fn r#type(&self) -> Type;

    /// For message and enum types, this is the name of the type.  If the name
    /// starts with a '.', it is fully-qualified.  Otherwise, C++-like scoping
    /// rules are used to find the type (i.e. first the nested types within this
    /// message are searched, then within the parent, on up to the root
    /// namespace).
    fn type_name(&self) -> String;

    /// For extensions, this is the name of the type being extended.  It is
    /// resolved in the same manner as `proto_type_name`.
    fn extendee(&self) -> String;

    /// JSON name of this field. The value is set by protocol compiler. If the
    /// user has set a "json_name" option on this field, that option's value
    /// will be used. Otherwise, it's deduced from the field's name by converting
    /// it to camelCase.
    fn json_name(&self) -> String;
    fn options(&self) -> I::FieldOptions;
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
    fn proto3_optional(&self) -> bool;

    fn is_well_known_type(&self) -> bool {
        self.well_known_type().is_some()
    }
    /// Alias for `r#type()`.
    /// If type_name is set, this need not be set.  If both this and type_name
    /// are set, this must be one of Enum, Message or Group.
    fn proto_type(&self) -> Type {
        self.r#type()
    }

    /// returns `true` if:
    ///
    /// - `syntax` is `Syntax::Proto3` and `proto3_optional` is `true`
    /// - `syntax` is `Syntax::Proto2` and `label` is `Label::Optional`.
    fn is_marked_optional(&self, syntax: Syntax) -> bool {
        match syntax {
            Syntax::Proto2 => self.label() == Label::Optional,
            Syntax::Proto3 => self.proto3_optional(),
        }
    }
    fn is_required(&self, syntax: Syntax) -> bool {
        syntax.supports_required_prefix() && self.label() == Label::Required
    }

    fn is_embed(&self) -> bool {
        matches!(self.r#type(), Type::Message)
    }

    fn is_enum(&self) -> bool {
        matches!(self.r#type(), Type::Enum)
    }

    fn is_scalar(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(_))
    }

    fn is_double(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Double))
    }
    fn is_float(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Float))
    }

    fn is_int64(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Int64))
    }

    fn is_uint64(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Uint64))
    }

    fn is_int32(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Int32))
    }

    fn is_fixed64(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Fixed64))
    }

    fn is_fixed32(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Fixed32))
    }

    fn is_bool(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Bool))
    }

    fn is_string(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::String))
    }

    fn is_bytes(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Bytes))
    }

    fn is_uint32(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Uint32))
    }

    fn is_sfixed32(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Sfixed32))
    }

    fn is_sfixed64(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Sfixed64))
    }

    fn is_sint32(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Sint32))
    }

    fn is_sint64(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Sint64))
    }
    fn is_message(&self) -> bool {
        matches!(self.r#type(), Type::Message)
    }

    fn is_repeated(&self) -> bool {
        self.label() == Label::Repeated
    }
}

/// Describes a service.
pub trait ServiceDescriptor<'a, I: Impl<'a>>: Clone + Copy + Debug {
    fn name(&self) -> &'a str;
    fn options(&self) -> I::ServiceOptions;
    fn methods(&self) -> I::MethodDescriptorIter;
}

/// Describes a method.
pub trait MethodDescriptor<'a, I: Impl<'a>>: Clone + Copy + Debug {
    fn name(&self) -> &'a str;
    /// Input type name.
    ///
    /// These are resolved in the same way as
    /// `FieldDescriptor.type_name`, but must refer to a message type
    fn input_type(&self) -> &'a str;
    /// Output type name.
    ///
    /// These are resolved in the same way as
    /// `FieldDescriptor.type_name`, but must refer to a message type
    fn output_type(&self) -> &'a str;
    /// Identifies if client streams multiple client messages
    fn client_streaming(&self) -> bool;
    /// Identifies if server streams multiple server messages
    fn server_streaming(&self) -> bool;
    fn options(&self) -> I::MethodOptions;
}

/// Describes a oneof.
// #[cfg_attr(test, automock)]
pub trait OneofDescriptor<'a, I: Impl<'a>>: Clone + Copy + Debug {
    fn name(&self) -> String;
    fn options(&self) -> I::OneofOptions;
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

// #[cfg_attr(test, automock)]
pub trait FileOptions<'a, I: Impl<'a>>: Clone + Copy + Debug {
    /// Java package where classes generated from this .proto will be
    /// placed.  By default, the proto package is used, but this is often
    /// inappropriate because proto packages do not normally start with backwards
    /// domain names.
    fn java_package(&self) -> String;
    /// If set, all the classes from the .proto file are wrapped in a single
    /// outer class with the given name.  This applies to both Proto1
    /// (equivalent to the old "--one_java_file" option) and Proto2 (where
    /// a .proto always translates to a single class, but you may want to
    /// explicitly choose the class name).
    fn java_outer_classname(&self) -> String;

    /// If set true, then the Java code generator will generate a separate .java
    /// file for each top-level message, enum, and service defined in the .proto
    /// file.  Thus, these types will *not* be nested inside the outer class
    /// named by java_outer_classname.  However, the outer class will still be
    /// generated to contain the file's getDescriptor() method as well as any
    /// top-level extensions defined in the file.
    fn java_multiple_files(&self) -> bool;

    /// If set true, then the Java2 code generator will generate code that
    /// throws an exception whenever an attempt is made to assign a non-UTF-8
    /// byte sequence to a string field.
    /// Message reflection will do the same.
    /// However, an extension field still accepts non-UTF-8 byte sequences.
    /// This option has no effect on when used with the lite runtime.    
    fn java_string_check_utf8(&self) -> bool;
    /// Generated classes can be optimized for speed or code size.
    fn optimize_for(&self) -> OptimizeMode;
    /// Sets the Go package where structs generated from this .proto will be
    /// placed. If omitted, the Go package will be derived from the following:
    ///   - The basename of the package import path, if provided.
    ///   - Otherwise, the package statement in the .proto file, if present.
    ///   - Otherwise, the basename of the .proto file, without extension.
    fn go_package(&self) -> String;
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
    fn cc_generic_services(&self) -> bool;

    fn java_generic_services(&self) -> bool;

    fn py_generic_services(&self) -> bool;

    fn php_generic_services(&self) -> bool;

    /// Is this file deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for everything in the file, or it will be completely ignored; in the very
    /// least, this is a formalization for deprecating files.
    fn deprecated(&self) -> bool;
    fn is_deprecated(&self) -> bool;
    /// Enables the use of arenas for the proto messages in this file. This applies
    /// only to generated classes for C++.
    fn cc_enable_arenas(&self) -> bool;
    /// Sets the objective c class prefix which is prepended to all objective c
    /// generated classes from this .proto. There is no default.
    fn objc_class_prefix(&self) -> String;

    /// Namespace for generated classes; defaults to the package.    
    fn csharp_namespace(&self) -> String;
    /// By default Swift generators will take the proto package and CamelCase it
    /// replacing '.' with underscore and use that to prefix the types/symbols
    /// defined. When this options is provided, they will use this value instead
    /// to prefix the types/symbols defined.
    fn swift_prefix(&self) -> String;

    /// Sets the php class prefix which is prepended to all php generated classes
    /// from this .proto. Default is empty.
    fn php_class_prefix(&self) -> String;

    /// Use this option to change the namespace of php generated classes. Default
    /// is empty. When this option is empty, the package name will be used for
    /// determining the namespace.
    fn php_namespace(&self) -> String;

    /// Use this option to change the namespace of php generated metadata classes.
    /// Default is empty. When this option is empty, the proto file name will be
    /// used for determining the namespace.
    fn php_metadata_namespace(&self) -> String;
    /// Use this option to change the package of ruby generated classes. Default
    /// is empty. When this option is not set, the package name will be used for
    /// determining the ruby package.
    fn ruby_package(&self) -> String;
    /// The parser stores options it doesn't recognize here.
    /// See the documentation for the "Options" section above.
    fn uninterpreted_options(&self) -> I::UninterpretedOptionIter;
}

// #[cfg_attr(test, automock)]
pub trait EnumValueOptions<'a, I: Impl<'a>>: Clone + Copy + Debug {
    /// Is this enum value deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the enum value, or it will be completely ignored; in the very least,
    /// this is a formalization for deprecating enum values.
    fn deprecated(&self) -> bool;
    fn is_deprecated(&self) -> bool;
    /// Options not recognized by the parser.
    fn uninterpreted_options(&self) -> I::UninterpretedOptionIter;
}

// #[cfg_attr(test, automock)]
pub trait MessageOptions<'a, I: Impl<'a>>: Clone + Copy + Debug {
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
    fn message_set_wire_format(&self) -> bool;
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
    fn map_entry(&self) -> bool;

    fn is_map_entry(&self) -> bool;

    fn deprecated(&self) -> bool;
    fn is_deprecated(&self) -> bool;
    fn no_standard_descriptor_accessor(&self) -> bool;
    /// The parser stores options it doesn't recognize here. See above.
    fn uninterpreted_options(&self) -> I::UninterpretedOptionsIter;
}

// #[cfg_attr(test, automock)]
pub trait FieldOptions<'a, I: Impl<'a>>: Clone + Copy + Debug {
    /// The ctype option instructs the C++ code generator to use a different
    /// representation of the field than it normally would.  See the specific
    /// options below.  This option is not yet implemented in the open source
    /// release -- sorry, we'll try to include it in a future version!
    fn c_type(&self) -> CType;
    /// The packed option can be enabled for repeated primitive fields to enable
    /// a more efficient representation on the wire. Rather than repeatedly
    /// writing the tag and type for each element, the entire array is encoded as
    /// a single length-delimited blob. In proto3, only explicit setting it to
    /// false will avoid using packed encoding.
    fn packed(&self) -> bool;
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
    fn js_type(&self) -> JsType;
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
    fn is_lazy(&self) -> bool;

    /// Is this field deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for accessors, or it will be completely ignored; in the very least, this
    /// is a formalization for deprecating fields.
    fn deprecated(&self) -> bool;
    fn is_deprecated(&self) -> bool;
    /// For Google-internal migration only. Do not use.
    fn is_weak(&self) -> bool;

    /// Options the parser does not recognize.
    fn uninterpreted_options(&self) -> I::UninterpretedOptionsIter;
}

// #[cfg_attr(test, automock)]
pub trait EnumOptions<'a, I: Impl<'a>>: Clone + Copy + Debug {
    /// Is this enum deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the enum, or it will be completely ignored; in the very least, this
    /// is a formalization for deprecating enums.
    fn deprecated(&self) -> bool;
    fn is_deprecated(&self) -> bool;
    /// Options not recognized by the parser.
    fn uninterpreted_options(&self) -> I::UninterpretedOptionsIter;
    /// Allows mapping different tag names to the same value.
    fn allow_alias(&self) -> bool;
}

/// Options for a Service.
///
/// Note: Field numbers 1 through 32 are reserved for Google's internal RPC
/// framework.
// #[cfg_attr(test, automock)]
pub trait ServiceOptions<'a, I: Impl<'a>>: Clone + Copy + Debug {
    /// Is this service deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the service, or it will be completely ignored; in the very least,
    /// this is a formalization for deprecating services.
    fn deprecated(&self) -> bool;
    fn is_deprecated(&self) -> bool;
    /// The parser stores options it doesn't recognize here. See above.
    fn uninterpreted_options(&self) -> I::UninterpretedOptionsIter;
}

/// Options for a Method.
///
/// Note:  Field numbers 1 through 32 are reserved for Google's internal RPC
/// framework.
// #[cfg_attr(test, automock)]
pub trait MethodOptions<'a, I: Impl<'a>>: Clone + Copy + Debug {
    // Note:  Field numbers 1 through 32 are reserved for Google's internal RPC
    //   framework.  We apologize for hoarding these numbers to ourselves, but
    //   we were already using them long before we decided to release Protocol
    //   Buffers.

    /// Is this method deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the method, or it will be completely ignored; in the very least,
    /// this is a formalization for deprecating methods.
    fn deprecated(&self) -> bool;
    fn is_deprecated(&self) -> bool;
    /// The parser stores options it doesn't recognize here. See above.
    fn uninterpreted_options(&self) -> I::UninterpretedOptionsIter;

    /// Is this method side-effect-free (or safe in HTTP parlance), or idempotent,
    /// or neither? HTTP based RPC implementation may choose GET verb for safe
    /// methods, and PUT verb for idempotent methods instead of the default POST.
    fn idempotency_level(&self) -> IdempotencyLevel;
}
// #[cfg_attr(test, automock)]
pub trait OneofOptions<'a, I: Impl<'a>>: Clone + Copy + Debug {
    /// The parser stores options it doesn't recognize here. See above.
    fn uninterpreted_options(&self) -> I::UninterpretedOptionsIter;
}

/// Range of reserved tag numbers. Reserved tag numbers may not be used by
/// fields or extension ranges in the same message. Reserved ranges may
/// not overlap.
// #[cfg_attr(test, automock)]
pub trait ReservedRange<'a, I: Impl<'a>>: Clone + Copy + Debug {
    /// Inclusive.
    fn start(&self) -> i32;

    /// Exclusive.
    fn end(&self) -> i32;

    fn in_range(&self, val: i32) -> bool {
        self.start() <= val && val < self.end()
    }
}

impl<'a, I: Impl<'a>, T: ReservedRange<'a, I>> PartialEq for T {
    fn eq(&self, other: &Self) -> bool {
        self.start() == other.start() && self.end() == other.end()
    }
}

pub trait ReservedRanges<'a, I: Impl<'a>>:
    Clone + Copy + Debug + IntoIterator<Item = I::ReservedRange, IntoIter = I::ReservedRangeIter>
{
    fn len(&self) -> usize {
        self.into_iter().len()
    }
    fn is_empty(&self) -> bool {
        self.into_iter().is_empty()
    }
    fn get(&self, index: usize) -> Option<I::ReservedRange> {
        self.into_iter().nth(index)
    }
    fn is_in_reserved_range(&self, num: i32) -> bool {
        self.into_iter().any(|r| r.start() <= num && r.end() >= num)
    }
}

pub trait ExtensionRange<'a, I: Impl<'a>>: Clone + Copy + Debug {
    /// Inclusive.
    fn start(&self) -> i32;
    /// Exclusive.
    fn end(&self) -> i32;
    fn in_range(&self, val: i32) -> bool {
        self.start() <= val && val < self.end()
    }
}

impl<'a, I: Impl<'a>, T: ExtensionRange<'a, I>> PartialEq for T {
    fn eq(&self, other: &I::ExtensionRange) -> bool {
        self.range.start() == other.start() && self.end() == other.end()
    }
}

/// Range of reserved numeric values. Reserved values may not be used by
/// entries in the same enum. Reserved ranges may not overlap.
///
/// Note that this is distinct from DescriptorProto.ReservedRange in that it
/// is inclusive such that it can appropriately represent the entire int32
/// domain.
// #[cfg_attr(test, automock)]
pub trait EnumReservedRange<'a, I: Impl<'a>>: Clone + Copy + Debug {
    /// Inclusive
    fn start(&self) -> i32;
    /// Inclusive
    fn end(&self) -> i32;
}

// #[cfg_attr(test, automock)]
pub trait EnumReservedRanges<'a, I: Impl<'a>>:
    IntoIterator<Item = I::EnumReservedRange, IntoIter = I::EnumReservedRangeIter>
{
    fn iter(&self) -> I::EnumReservedRangeIter {
        self.ranges.into()
    }
    fn len(&self) -> usize {
        self.ranges.len()
    }
    fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }
    fn get(&self, index: usize) -> Option<I::EnumReservedRange> {
        self.ranges.get(index).map(|r| r.into())
    }
    fn is_range_reserved(&self, min: i32, max: i32) -> bool {
        self.iter().any(|r| r.start() <= min && r.end() >= max)
    }
    fn is_in_reserved_range(&self, num: i32) -> bool {
        self.iter().any(|r| r.start() <= num && r.end() >= num)
    }
}

/// A message representing a option the parser does not recognize. This only
/// appears in options protos created by the compiler::Parser class.
/// DescriptorPool resolves these when building Descriptor objects. Therefore,
/// options protos in descriptor objects (e.g. returned by Descriptor::options(),
/// or produced by Descriptor::CopyTo()) will never have UninterpretedOptions
/// in them.
pub trait UninterpretedOption<'a, I: Impl<'a>>:
    core::fmt::Debug + Copy + Clone + IntoIterator<Item = I::NamePart, IntoIter = I::NamePartIter>
{
    type Impl: crate::Impl<'a>;
    fn name_parts(&self) -> I::NameParts;
    fn identifier_value(&self) -> &'a str;
    fn positive_int_value(&self) -> u64;
    fn negative_int_value(&self) -> i64;
    fn double_value(&self) -> f64;
    fn string_value(&self) -> &'a [u8];
    fn aggregate_value(&self) -> &'a str;
}

/// The name of the uninterpreted option.  Each string represents a segment in
/// a dot-separated name. `is_extension` is `true` if a segment represents an
/// extension (denoted with parentheses in options specs in .proto files).
///
/// E.g.,
/// ```norun
/// "foo.(bar.baz).qux" => [ ("foo", false), ("bar.baz", true), ("qux", false) ]
/// ```
pub trait NameParts<'a, I: Impl<'a>>:
    Copy + Clone + IntoIterator<Item = I::NamePart, IntoIter = I::NamePartIter>
{
    type Impl: crate::Impl<'a>;

    fn len(&self) -> usize {
        self.into_iter().len()
    }
    fn is_empty(&self) -> bool {
        self.into_iter().is_empty()
    }
    fn formatted_value(&self) -> String {
        self.into_iter()
            .map(|x| x.formatted_value())
            .collect::<Vec<String>>()
            .join(".")
    }
}
impl<'a, I: Impl<'a>, T: NameParts<'a, I>> ToString for T {
    fn to_string(&self) -> String {
        self.value()
    }
}
impl<'a, I: Impl<'a>, T: NameParts<'a, I>> Display for T {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.formatted_value())
    }
}
impl<'a, I: Impl<'a>, T: NameParts<'a, I>> Debug for T {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.formatted_value())
    }
}

pub trait NamePart<'a, I: Impl<'a>> {
    /// the value of the part
    /// E.g. `"foo"`, `"bar.baz"`, or `"qux"` of:
    /// ```no_run
    /// "foo.(bar.baz).qux" => [ ("foo", false), ("bar.baz", true), ("qux", false) ]
    /// ```
    fn value(&self) -> &'a str;
    fn is_extension(&self) -> bool;
    fn as_str(&self) -> &'a str {
        self.value()
    }
    fn formatted_value(&self) -> String {
        if self.is_extension() {
            format!("({})", self.value())
        } else {
            self.value().to_string()
        }
    }
}

impl<'a, I: Impl<'a>, T: NamePart<'a, I>> core::fmt::Debug for T {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.value())
    }
}
impl<'a, I: Impl<'a>, T: NamePart<'a, I>> ToString for T {
    fn to_string(&self) -> String {
        self.value().to_string()
    }
}
impl<'a, I: Impl<'a>, T: NamePart<'a, I>> Display for T {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.part.is_extension {
            write!(f, "({})", self.value())
        } else {
            write!(f, "{}", self.value())
        }
    }
}

impl<'a, I: Impl<'a>, T: NamePart<'a, I>> Eq for T {}

impl<'a, I: Impl<'a>, T: NamePart<'a, I>> PartialEq<T> for T {
    fn eq(&self, other: &Self) -> bool {
        self.part == other.part
    }
}

impl<'a, I: Impl<'a>, T: NamePart<'a, I>> Debug for T {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NamePart")
            .field("value", &self.value())
            .field("is_extension", &self.is_extension())
            .finish()
    }
}

impl<'a, I: Impl<'a>, T: NamePart<'a, I>> PartialEq<String> for T {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other
    }
}
impl<'a, I: Impl<'a>, T: NamePart<'a, I>> PartialEq<&str> for T {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl<'a, I: Impl<'a>, T: NamePart<'a, I>> PartialEq<T> for str {
    fn eq(&self, other: I::NamePart) -> bool {
        self.as_str() == *other
    }
}

impl<'a, I: Impl<'a>, T: NamePart<'a, I>> Deref for T {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.value()
    }
}

pub trait SourceCodeInfo<'a, I: Impl<'a>>:
    IntoIterator<Item = I::Location, IntoIter = I::LocationIter>
{
    type Impl: crate::Impl<'a>;
    fn iter(&self) -> I::LocationIter {
        self.into_iter()
    }
    fn len(&self) -> usize {
        self.into_iter().len()
    }
    fn is_empty(&self) -> bool {
        self.into_iter().len() == 0
    }
}

/// Comments associated to entities in the source code.
pub trait Location<'a, I: Impl<'a>> {
    /// Identifies which part of the FileDescriptorProto was defined at this
    /// location.
    ///
    /// Each element is a field number or an index.  They form a path from
    /// the root FileDescriptorProto to the place where the definition.  For
    /// example, this path:
    ///   [ 4, 3, 2, 7, 1 ]
    /// refers to:
    ///   file.message_type(3)  // 4, 3
    ///       .field(7)         // 2, 7
    ///       .name()           // 1
    /// This is because FileDescriptorProto.message_type has field number 4:
    ///   repeated DescriptorProto message_type = 4;
    /// and DescriptorProto.field has field number 2:
    ///   repeated FieldDescriptorProto field = 2;
    /// and FieldDescriptorProto.name has field number 1:
    ///   optional string name = 1;
    ///
    /// Thus, the above path gives the location of a field name.  If we removed
    /// the last element:
    ///   [ 4, 3, 2, 7 ]
    /// this path refers to the whole field declaration (from the beginning
    /// of the label to the terminating semicolon).
    fn path(&self) -> &'a [i32];

    /// Always has exactly three or four elements: start line, start column,
    /// end line (optional, otherwise assumed same as start line), end column.
    /// These are packed into a single field for efficiency.  Note that line
    /// and column numbers are zero-based -- typically you will want to add
    /// 1 to each before displaying to a user
    fn span(&self) -> &'a [i32];

    /// Returns any comment immediately preceding the node, without anyElsewhere
    /// whitespace between it and the comment.
    fn leading_comments(&self) -> &'a str;

    /// Returns each comment block or line above the
    /// entity but separated by whitespace.a
    fn leading_detached_comments(&self) -> std::slice::Iter<'a, String>;
    /// Returns any comment immediately following the entity, without any
    /// whitespace between it and the comment. If the comment would be a leading
    /// comment for another entity, it won't be considered a trailing comment.
    fn trailing_comments(&self) -> &'a str;

    fn is_file_syntax_location(&self) -> bool {
        self.path().len() == 1 && FileDescriptorPath::Syntax == self.path()[0]
    }

    fn is_file_package_location(&self) -> bool {
        self.path().len() == 1 && FileDescriptorPath::Package == self.path()[0]
    }

    fn file_descriptor_path(&self) -> Result<FileDescriptorPath, anyhow::Error> {
        FileDescriptorPath::try_from(self.path().get(0))
    }

    fn has_comments(&self) -> bool {
        !self.leading_comments().is_empty()
            || self.leading_detached_comments().count() > 0
            || !self.trailing_comments().is_empty()
    }
}