use crate::lang::{Keyword, Lang};
use anyhow::bail;
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Unspecified;

impl Lang for Unspecified {
    type Error = anyhow::Error;
    fn is_keyword<T: ToString>(&self, s: T) -> Result<Keyword, Self::Error> {
        bail!("Language not specified");
    }

    fn name() -> &'static str {
        "not_specified"
    }
}
