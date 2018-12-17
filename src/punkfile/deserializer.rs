use super::serde_json::Value;
use std::fs::File;

#[derive(Deserialize)]
pub struct PunkFileJSON {
    magic_number: String,
    pub classes: Vec<ClassDeserialize>,
    pub main_fields: Vec<CodeDeserialize>,
    pub main_code: Vec<String>
}

#[derive(Deserialize)]
pub struct ClassDeserialize {
    pub constant_pool: Vec<Value>,
    pub this: usize,
    #[serde(rename = "super")]
    pub super_cls: usize,
    pub fields: Vec<CodeDeserialize>,
    pub methods: Vec<CodeDeserialize>
}

#[derive(Deserialize)]
pub struct CodeDeserialize {
    name_index: usize,
    descriptor_index: usize,
    code: Vec<String>
}

impl PunkFileJSON {
    pub fn from_file(uri: &str) -> PunkFileJSON {
        let file = match File::open(uri) {
            Ok(file) => file,
            Err(_) => panic!("The json file could not be opened")
        };
        match serde_json::from_reader(file) {
            Ok(v) => return v,
            Err(err) => panic!("The json from {} could not be parsed\n {}", uri, err)
        };
    }
}