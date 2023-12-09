use anyhow::bail;

/// Paths for nodes in a FileDescriptorProto

/// TODO: This could/should be turned into a tree of nested enums, where leaves end in something like Node(i32)

#[derive(Clone, PartialEq, Eq, Copy)]
#[repr(i32)]
pub enum FileDescriptorPath {
    /// file name, relative to root of source tree
    Name = 1,
    /// FileDescriptorProto.package
    Package = 2,
    /// Names of files imported by this file.
    Dependency = 3,

    /// Indexes of the public imported files in the dependency list above.
    PublicDependency = 10,

    /// Indexes of the weak imported files in the dependency list.
    /// For Google-internal migration only. Do not use.
    WeakDependency = 11,

    // All top-level definitions in this file.
    MessageType = 4,
    /// FileDescriptorProto.enum_type
    EnumType = 5,
    /// FileDescriptorProto.service
    Service = 6,
    /// FileDescriptorProto.extension
    Extension = 7,

    Options = 8,
    /// This field contains optional information about the original source code.
    /// You may safely remove this entire field without harming runtime
    /// functionality of the descriptors -- the information is needed only by
    /// development tools.
    SourceCodeInfo = 9,

    /// FileDescriptorProto.syntax
    Syntax = 12,
}

impl TryFrom<Option<&i32>> for FileDescriptorPath {
    type Error = anyhow::Error;
    fn try_from(v: Option<&i32>) -> Result<Self, Self::Error> {
        match v {
            Some(v) => Self::try_from(*v),
            None => bail!("value is empty and can not be converted to FileDescriptorPath"),
        }
    }
}

impl TryFrom<i32> for FileDescriptorPath {
    type Error = anyhow::Error;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == FileDescriptorPath::Name as i32 => Ok(FileDescriptorPath::Name),
            x if x == FileDescriptorPath::Package as i32 => Ok(FileDescriptorPath::Package),
            x if x == FileDescriptorPath::Dependency as i32 => Ok(FileDescriptorPath::Dependency),
            x if x == FileDescriptorPath::PublicDependency as i32 => {
                Ok(FileDescriptorPath::PublicDependency)
            }
            x if x == FileDescriptorPath::WeakDependency as i32 => {
                Ok(FileDescriptorPath::WeakDependency)
            }
            x if x == FileDescriptorPath::MessageType as i32 => Ok(FileDescriptorPath::MessageType),
            x if x == FileDescriptorPath::EnumType as i32 => Ok(FileDescriptorPath::EnumType),
            x if x == FileDescriptorPath::Service as i32 => Ok(FileDescriptorPath::Service),
            x if x == FileDescriptorPath::Extension as i32 => Ok(FileDescriptorPath::Extension),
            x if x == FileDescriptorPath::Options as i32 => Ok(FileDescriptorPath::Options),
            x if x == FileDescriptorPath::SourceCodeInfo as i32 => {
                Ok(FileDescriptorPath::SourceCodeInfo)
            }
            x if x == FileDescriptorPath::Syntax as i32 => Ok(FileDescriptorPath::Syntax),
            _ => bail!("invalid FileDescriptorPath: {}", v),
        }
    }
}

impl PartialEq<i32> for FileDescriptorPath {
    fn eq(&self, other: &i32) -> bool {
        *other == *self as i32
    }
}
impl PartialEq<FileDescriptorPath> for i32 {
    fn eq(&self, other: &FileDescriptorPath) -> bool {
        *other == *self
    }
}
/// Paths for nodes in a DescriptorProto
#[derive(Clone, PartialEq, Eq, Copy)]
pub enum DescriptorPath {
    /// DescriptorProto.field
    Field = 2,
    /// DescriptorProto.nested_type
    NestedType = 3,
    /// DescriptorProto.enum_type
    EnumType = 4,

    Extension = 6,

    /// DescriptorProto.oneof_decl
    OneofDecl = 8,
}

impl TryFrom<i32> for DescriptorPath {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == DescriptorPath::Field as i32 => Ok(DescriptorPath::Field),
            x if x == DescriptorPath::NestedType as i32 => Ok(DescriptorPath::NestedType),
            x if x == DescriptorPath::EnumType as i32 => Ok(DescriptorPath::EnumType),
            x if x == DescriptorPath::OneofDecl as i32 => Ok(DescriptorPath::OneofDecl),
            _ => Err(()),
        }
    }
}

impl PartialEq<i32> for DescriptorPath {
    fn eq(&self, other: &i32) -> bool {
        *other == *self
    }
}
impl PartialEq<DescriptorPath> for i32 {
    fn eq(&self, other: &DescriptorPath) -> bool {
        *other == *self
    }
}

/// Paths for nodes in an EnumDescriptorProto

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EnumDescriptorPath {
    /// EnumDescriptorProto.Value
    Value = 2,
}
impl PartialEq<i32> for EnumDescriptorPath {
    fn eq(&self, other: &i32) -> bool {
        *other == *self as i32
    }
}
impl PartialEq<EnumDescriptorPath> for i32 {
    fn eq(&self, other: &EnumDescriptorPath) -> bool {
        *other == *self
    }
}

impl TryFrom<i32> for EnumDescriptorPath {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == EnumDescriptorPath::Value as i32 => Ok(EnumDescriptorPath::Value),
            _ => Err(()),
        }
    }
}

// Paths for nodes in an ServiceDescriptorProto
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ServiceDescriptorPath {
    /// ServiceDescriptorProto.method
    Method = 2,
    Mixin = 6,
}
impl TryFrom<i32> for ServiceDescriptorPath {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == EnumDescriptorPath::Value as i32 => Ok(ServiceDescriptorPath::Method),
            _ => Err(()),
        }
    }
}
impl PartialEq<i32> for ServiceDescriptorPath {
    fn eq(&self, other: &i32) -> bool {
        *other == *self as i32
    }
}
impl PartialEq<ServiceDescriptorPath> for i32 {
    fn eq(&self, other: &ServiceDescriptorPath) -> bool {
        *other as i32 == *self
    }
}
