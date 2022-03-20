use std::{cell::RefCell, collections::HashMap, fmt::Debug, ops, rc::Rc};

const OUTPUT_PATH_KEY: &str = "output_path";

pub(crate) type ParamMutatorFn = Rc<RefCell<dyn FnMut(&mut Parameters)>>;

#[derive(Clone, Debug, Default)]
pub struct Parameters {
    map: HashMap<String, String>,
}
impl Parameters {
    pub fn new(params: Option<&str>) -> Self {
        params.map_or(Self::default(), |val| Self::parse(val))
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.map.get(key).cloned()
    }
    pub fn len(&self) -> usize {
        self.map.len()
    }
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
    pub fn contains_key(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }
    pub fn iter(&self) -> std::collections::hash_map::Iter<String, String> {
        self.map.iter()
    }
    pub fn output_path(&self) -> String {
        self.get(OUTPUT_PATH_KEY).unwrap_or_else(|| ".".to_string())
    }
    pub fn set_output_path(&mut self, path: String) {
        self.map.insert(OUTPUT_PATH_KEY.to_string(), path);
    }
    pub fn set_param(
        &mut self,
        key: impl std::ops::Deref<Target = str>,
        value: impl std::ops::Deref<Target = str>,
    ) {
        self.map
            .insert(key.deref().to_string(), value.deref().to_string());
    }
    pub fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }
    pub fn parse(val: &str) -> Self {
        let mut map = HashMap::new();
        for param in val.split(',') {
            if param.contains('=') {
                let parts = param.splitn(2, '=').collect::<Vec<_>>();
                map.insert(parts[0].to_string(), parts[1].to_string());
            } else {
                map.insert(param.to_string(), "".to_string());
            };
        }
        Self { map }
    }
}

impl std::fmt::Display for Parameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.map.fmt(f)
    }
}
impl From<String> for Parameters {
    fn from(s: String) -> Self {
        Self::new(Some(&s))
    }
}
impl From<&str> for Parameters {
    fn from(s: &str) -> Self {
        Self::new(Some(s))
    }
}

impl From<Option<String>> for Parameters {
    fn from(s: Option<String>) -> Self {
        Self::new(s.as_ref().map(|s| s.as_str()))
    }
}
// impl<T> From<Option<T>> for Parameters
// where
//     T: std::ops::Deref<Target = str>,
// {
//     fn from(s: Option<T>) -> Self {
//         Self::new(s.as_deref())
//     }
// }

impl From<Option<&String>> for Parameters {
    fn from(s: Option<&String>) -> Self {
        Self::new(s.map(|s| s.as_str()))
    }
}
