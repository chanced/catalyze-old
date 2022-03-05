#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    Scalar(Scalar),
    Enum,
    Message,
    /// not supported
    Group,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Scalar {
    Double = 1,
    Float = 2,
    /// Not ZigZag encoded.  Negative numbers take 10 bytes.  Use TYPE_SINT64 if
    /// negative values are likely.
    Int64 = 3,
    Uint64 = 4,
    /// Not ZigZag encoded.  Negative numbers take 10 bytes.  Use TYPE_SINT32 if
    /// negative values are likely.
    Int32 = 5,
    Fixed64 = 6,
    Fixed32 = 7,
    Bool = 8,
    String = 9,
    /// New in version 2.
    Bytes = 12,
    Uint32 = 13,
    Enum = 14,
    Sfixed32 = 15,
    Sfixed64 = 16,
    /// Uses ZigZag encoding.
    Sint32 = 17,
    /// Uses ZigZag encoding.
    Sint64 = 18,
}

impl TryFrom<Option<i32>> for Type {}

impl TryFrom<i32> for Type {}
impl<T: Into<i32>> TryFrom<T> for Type {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum CType {
    /// Default mode.
    String = 0,
    Cord = 1,
    StringPiece = 2,
}

impl TryFrom<Option<i32>> for CType {}
impl TryFrom<i32> for CType {}
impl<T: Into<i32>> TryFrom<T> for CType {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum JsType {
    /// Use the default type.
    JsNormal = 0,
    /// Use JavaScript strings.
    JsString = 1,
    /// Use JavaScript numbers.
    JsNumber = 2,
}
impl TryFrom<Option<i32>> for CType {}
impl TryFrom<i32> for CType {}
impl<T: Into<i32>> TryFrom<T> for CType {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum Label {
    Required = 1,
    Optional = 2,
    Repeated = 3,
}
impl TryFrom<Option<i32>> for Label {}
impl TryFrom<i32> for Label {}
impl<T: Into<i32>> TryFrom<T> for Label {}

/// Generated classes can be optimized for speed or code size.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum OptimizeMode {
    /// Generate complete code for parsing, serialization,
    Speed = 1,
    /// etc.
    ///
    /// Use ReflectionOps to implement these methods.
    CodeSize = 2,
    /// Generate code using MessageLite and the lite runtime.
    LiteRuntime = 3,
}
impl TryFrom<Option<i32>> for OptimizeMode {}
impl TryFrom<i32> for OptimizeMode {}
impl<T: Into<i32>> TryFrom<T> for OptimizeMode {}

/// Is this method side-effect-free (or safe in HTTP parlance), or idempotent,
/// or neither? HTTP based RPC implementation may choose GET verb for safe
/// methods, and PUT verb for idempotent methods instead of the default POST.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum IdempotencyLevel {
    IdempotencyUnknown = 0,
    /// implies idempotent
    NoSideEffects = 1,
    /// idempotent, but may have side effects
    Idempotent = 2,
}

impl TryFrom<Option<i32>> for IdempotencyLevel {}
impl TryFrom<i32> for IdempotencyLevel {}
impl<T: Into<i32>> TryFrom<T> for IdempotencyLevel {}
