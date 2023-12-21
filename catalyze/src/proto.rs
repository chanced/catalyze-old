#[derive(Debug, Clone, Copy)]
pub struct SourceCodeInfo {
    pub(crate) info: &'a protobuf::descriptor::SourceCodeInfo,
}

impl SourceCodeInfo {
    pub fn iter(&self) -> LocationIter {
        self.into()
    }
    pub fn len(&self) -> usize {
        self.info.location.len()
    }
    pub fn is_empty(&self) -> bool {
        self.info.location.is_empty()
    }
}

impl From<&'a protobuf::descriptor::SourceCodeInfo> for SourceCodeInfo {
    fn from(info: &'a protobuf::descriptor::SourceCodeInfo) -> Self {
        SourceCodeInfo { info }
    }
}

impl From<Option<&'a protobuf::descriptor::SourceCodeInfo>> for SourceCodeInfo {
    fn from(info: Option<&'a protobuf::descriptor::SourceCodeInfo>) -> Self {
        SourceCodeInfo {
            info: info.unwrap_or(&DEFAULT_SOURCE_CODE_INFO),
        }
    }
}

impl IntoIterator for SourceCodeInfo {
    type Item = Location;
    type IntoIter = LocationIter;

    fn into_iter(self) -> Self::IntoIter {
        LocationIter::from(&self)
    }
}

impl Default for Location {
    fn default() -> Self {
        Location {
            loc: &DEFAULT_LOCATION,
        }
    }
}

// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[test]
//     fn test_to_string() {
//         let mut p1 = protobuf::descriptor::uninterpreted_option::NamePart::new();
//         p1.set_name_part("foo".to_string());
//         let mut p2 = protobuf::descriptor::uninterpreted_option::NamePart::new();
//         p2.set_name_part("bar".to_string());
//         p2.set_is_extension(true);
//         let mut p3 = protobuf::descriptor::uninterpreted_option::NamePart::new();
//         p3.set_name_part("baz".to_string());
//         let parts = vec![p1, p2, p3];
//         let name_parts: NameParts<'_> = NameParts::from(&parts);
//         assert_eq!(name_parts.to_string(), "foo.(bar).baz");
//         assert_eq!(name_parts.get(0).unwrap(), "foo")
//     }
// }

// /// Comments associated to entities in the source code.
// #[derive(Debug, Clone, Copy)]
// pub struct Location {
//     loc: protobuf::descriptor::source_code_info::Location,
// }
// impl From<protobuf::descriptor::source_code_info::Location> for Location {
//     fn from(loc: protobuf::descriptor::source_code_info::Location) -> Self {
//         Self { loc }
//     }
// }

// impl Location {
//     /// Identifies which part of the FileDescriptorProto was defined at this
//     /// location.
//     ///
//     /// Each element is a field number or an index.  They form a path from
//     /// the root FileDescriptorProto to the place where the definition.  For
//     /// example, this path:
//     ///   [ 4, 3, 2, 7, 1 ]
//     /// refers to:
//     ///   file.message_type(3)  // 4, 3
//     ///       .field(7)         // 2, 7
//     ///       .name()           // 1
//     /// This is because FileDescriptorProto.message_type has field number 4:
//     ///   repeated DescriptorProto message_type = 4;
//     /// and DescriptorProto.field has field number 2:
//     ///   repeated FieldDescriptorProto field = 2;
//     /// and FieldDescriptorProto.name has field number 1:
//     ///   optional string name = 1;
//     ///
//     /// Thus, the above path gives the location of a field name.  If we removed
//     /// the last element:
//     ///   [ 4, 3, 2, 7 ]
//     /// this path refers to the whole field declaration (from the beginning
//     /// of the label to the terminating semicolon).
//     pub fn path(&self) -> &'a [i32] {
//         &self.loc.path
//     }
//     /// Always has exactly three or four elements: start line, start column,
//     /// end line (optional, otherwise assumed same as start line), end column.
//     /// These are packed into a single field for efficiency.  Note that line
//     /// and column numbers are zero-based -- typically you will want to add
//     /// 1 to each before displaying to a user
//     pub fn span(&self) -> &'a [i32] {
//         &self.loc.span
//     }

//     /// Returns any comment immediately preceding the node, without anyElsewhere
//     /// whitespace between it and the comment.
//     pub fn leading_comments(&self) -> &str {
//         self.loc.leading_comments()
//     }

//     /// Returns each comment block or line above the
//     /// entity but separated by whitespace.a
//     pub fn leading_detached_comments(&self) -> std::slice::Iter<'a, String> {
//         self.loc.leading_detached_comments.iter()
//     }
//     /// Returns any comment immediately following the entity, without any
//     /// whitespace between it and the comment. If the comment would be a leading
//     /// comment for another entity, it won't be considered a trailing comment.
//     pub fn trailing_comments(&self) -> &str {
//         self.loc.trailing_comments()
//     }

//     pub fn is_file_syntax_location(&self) -> bool {
//         self.path().len() == 1 && FileDescriptorPath::Syntax == self.path()[0]
//     }

//     pub fn is_file_package_location(&self) -> bool {
//         self.path().len() == 1 && FileDescriptorPath::Package == self.path()[0]
//     }

//     pub fn file_descriptor_path(&self) -> Result<FileDescriptorPath, crate::error::Error> {
//         FileDescriptorPath::try_from(self.path().first())
//     }

//     pub fn has_comments(&self) -> bool {
//         !self.leading_comments().is_empty()
//             || self.leading_detached_comments().count() > 0
//             || !self.trailing_comments().is_empty()
//     }
// }

// #[cfg(test)]

// mod test_data {

//     lazy_static::lazy_static! {
//         pub static ref DEFAULT_FILE_DESCRIPTOR_PROTO:protobuf::descriptor::FileDescriptorProto = protobuf::descriptor::FileDescriptorProto::default();
//         pub static ref DEFAULT_DESCRIPTOR_PROTO:protobuf::descriptor::DescriptorProto = protobuf::descriptor::DescriptorProto::default();
//         pub static ref DEFAULT_FIELD_DESCRIPTOR_PROTO:protobuf::descriptor::FieldDescriptorProto = protobuf::descriptor::FieldDescriptorProto::default();
//         pub static ref DEFAULT_ENUM_DESCRIPTOR_PROTO:protobuf::descriptor::EnumDescriptorProto = protobuf::descriptor::EnumDescriptorProto::default();
//         pub static ref DEFAULT_ENUM_VALUE_DESCRIPTOR_PROTO:protobuf::descriptor::EnumValueDescriptorProto = protobuf::descriptor::EnumValueDescriptorProto::default();
//         pub static ref DEFAULT_SERVICE_DESCRIPTOR_PROTO:protobuf::descriptor::ServiceDescriptorProto = protobuf::descriptor::ServiceDescriptorProto::default();
//         pub static ref DEFAULT_METHOD_DESCRIPTOR_PROTO:protobuf::descriptor::MethodDescriptorProto = protobuf::descriptor::MethodDescriptorProto::default();
//         pub static ref DEFAULT_ONEOF_DESCRIPTOR:protobuf::descriptor::OneofDescriptorProto = protobuf::descriptor::OneofDescriptorProto::default();
//     }
// }
