/// Paths for nodes in a FileDescriptorProto
#[derive(Clone, Copy)]
pub enum FileDescriptorPath {
    /// FileDescriptorProto.package
    Package = 2,
    /// FileDescriptorProto.message_type
    MessageType = 4,
    /// FileDescriptorProto.enum_type
    EnumType = 5,
    /// FileDescriptorProto.service
    Service = 6,
    /// FileDescriptorProto.syntax
    Syntax = 12,
}
impl TryFrom<i32> for FileDescriptorPath {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == FileDescriptorPath::Package as i32 => Ok(FileDescriptorPath::Package),
            x if x == FileDescriptorPath::MessageType as i32 => Ok(FileDescriptorPath::MessageType),
            x if x == FileDescriptorPath::EnumType as i32 => Ok(FileDescriptorPath::EnumType),
            x if x == FileDescriptorPath::Service as i32 => Ok(FileDescriptorPath::Service),
            x if x == FileDescriptorPath::Syntax as i32 => Ok(FileDescriptorPath::Syntax),
            _ => Err(()),
        }
    }
}

/// Paths for nodes in a DescriptorProto
#[derive(Clone, Copy)]
pub enum DescriptorPath {
    /// DescriptorProto.field
    Field = 2,
    /// DescriptorProto.nested_type
    NestedType = 3,
    /// DescriptorProto.enum_type
    EnumType = 4,
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

/// Paths for nodes in an EnumDescriptorProto

#[derive(Clone, Copy)]
pub enum EnumDescriptorPath {
    /// EnumDescriptorProto.Value
    Value = 2,
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
#[derive(Clone, Copy)]
pub enum ServiceDescriptorPath {
    /// ServiceDescriptorProto.method
    Method = 2,
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
