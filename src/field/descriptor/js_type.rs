#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum JsType {
    /// Use the default type.
    JsNormal = 0,
    /// Use JavaScript strings.
    JsString = 1,
    /// Use JavaScript numbers.
    JsNumber = 2,
}
impl From<prost_types::field_options::JsType> for JsType {
    fn from(js_type: prost_types::field_options::CType) -> Self {
        match js_type {
            prost_types::field_options::JsType::JsNormal => JsType::JsNormal,
            prost_types::field_options::JsType::JsString => JsType::JsString,
            prost_types::field_options::JsType::JsNumber => JsType::JsNumber,
        }
    }
}
