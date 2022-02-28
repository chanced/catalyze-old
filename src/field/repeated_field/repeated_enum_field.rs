#[derive(Debug, Clone)]
pub struct RepeatedEnumField<'a, U>(EnumFieldDetail<'a, U>);

impl<'a, U> RepeatedEnumField<'a, U> {
    pub fn name(&self) -> Name<U> {
        self.0.detail.name()
    }
    pub fn fully_qualified_name(&self) -> &str {
        self.0.detail.fully_qualified_name()
    }
}

impl<'a, U> FullyQualified for RepeatedEnumField<'a, U> {
    fn fully_qualified_name(&self) -> &str {
        self.detail.fully_qualified_name()
    }
}
