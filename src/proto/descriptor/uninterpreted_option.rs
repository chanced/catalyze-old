use std::marker::PhantomData;

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
    u: PhantomData<U>,
}
impl<'a, U> Clone for UninterpretedOption<'a, U> {
    fn clone(&self) -> Self {
        UninterpretedOption {
            opt: self.opt,
            u: PhantomData,
        }
    }
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
