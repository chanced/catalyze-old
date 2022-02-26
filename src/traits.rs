pub(crate) trait Upgrade: Sized {
    type Target;
    fn upgrade(self) -> Self::Target;
}

pub(crate) trait Downgrade: Sized {
    type Target;
    fn downgrade(self) -> Self::Target;
}
