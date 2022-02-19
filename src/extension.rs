use crate::Name;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Extension<U> {
    name: Name<U>,
    placeholder_until_i_fill_in_this_field: String,
    // TODO: see above.
}
