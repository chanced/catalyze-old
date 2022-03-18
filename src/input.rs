use std::collections::HashMap;

const OUTPUT_PATH_KEY: &str = "output_path";

#[derive(Clone, Debug, Default)]
pub struct Parameters {
    table: HashMap<String, String>,
}
impl Parameters {
    pub fn new(params: &str) -> Self {
        let table = Parameters::parse_table(params);
        Self { table }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.table.get(key).cloned()
    }
    pub fn len(&self) -> usize {
        self.table.len()
    }
    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }
    pub fn contains_key(&self, key: &str) -> bool {
        self.table.contains_key(key)
    }
    pub fn iter(&self) -> std::collections::hash_map::Iter<String, String> {
        self.table.iter()
    }
    pub fn output_path(&self) -> String {
        self.get(OUTPUT_PATH_KEY).unwrap_or_else(|| ".".to_string())
    }
    pub fn set_output_path(&mut self, path: String) {
        self.table.insert(OUTPUT_PATH_KEY.to_string(), path);
    }
    pub fn insert(&mut self, path: &str) {
        self.table.insert(path.to_string(), path.to_string());
    }
    fn parse_table(val: &str) -> HashMap<String, String> {
        let mut params = HashMap::new();
        for param in val.split(',') {
            if param.contains('=') {
                let parts = param.splitn(2, '=').collect::<Vec<_>>();
                params.insert(parts[0].to_string(), parts[1].to_string());
            } else {
                params.insert(param.to_string(), "".to_string());
            }
        }
        params
    }
}

impl From<String> for Parameters {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}
impl From<&str> for Parameters {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

#[derive(Clone, Debug)]
pub enum DecodedInput {
    CodeGeneratorRequest(prost_types::compiler::CodeGeneratorRequest),
    FileDescriptorSet(prost_types::FileDescriptorSet),
}
#[derive(Clone, Debug)]
pub struct Input {
    pub files: Vec<prost_types::FileDescriptorProto>,
    pub targets: Vec<String>,
    pub parmeters: Parameters,
    pub protoc_version: Option<semver::Version>,
}

impl Input {
    pub fn new(files: Vec<prost_types::FileDescriptorProto>, params: &str) -> Self {
        let parmeters = Parameters::new(params);
        Self {
            files,
            targets: vec![],
            parmeters,
            protoc_version: None,
        }
    }
    pub fn files(&self) -> std::slice::Iter<prost_types::FileDescriptorProto> {
        self.files.iter()
    }
}
