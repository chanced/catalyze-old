use crate::util::Util;

use super::NameParts;

/// A message representing a option the parser does not recognize. This only
/// appears in options protos created by the compiler::Parser class.
/// DescriptorPool resolves these when building Descriptor objects. Therefore,
/// options protos in descriptor objects (e.g. returned by Descriptor::options(),
/// or produced by Descriptor::CopyTo()) will never have UninterpretedOptions
/// in them.
#[derive(Debug)]
pub struct UninterpretedOption<'a, U> {
    opt: &'a prost_types::UninterpretedOption,
    util: Util<U>,
}
impl<'a, U> Clone for UninterpretedOption<'a, U> {
    fn clone(&self) -> Self {
        UninterpretedOption {
            opt: self.opt,
            util: self.util.clone(),
        }
    }
}
impl<'a, U> UninterpretedOption<'a, U> {
    pub fn name_parts(&self) -> NameParts<'a, U> {
        NameParts::new(&self.opt.name, self.util.clone())
    }
}
