use crate::lang::{Lang,Keyword};
use anyhow::bail;
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct NotSpecified;

impl Lang for NotSpecified {
    fn is_keyword<T: ToString>(&self, s: T) -> anyhow::Result<Keyword> {
        bail!("Language not specified");
    }

    fn name() -> &'static str {
        "not_specified"
    }
}
