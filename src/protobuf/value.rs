pub struct Struct {
    fields: BTreeMap<String, Value>,
}

pub enum Value {
    Null(i32),
    Number(f64),
    String(String),
    Bool(bool),
    Struct(Struct),
}
