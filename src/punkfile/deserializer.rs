use super::serde_json::Value;
use std::fs::File;

#[derive(Deserialize)]
pub struct PunkFileJSON {
    magic_number: String,
    pub classes: Vec<ClassDeserialize>,
    pub main_code: Vec<String>
}

#[derive(Deserialize)]
pub struct ClassDeserialize {
    pub this: String,
    #[serde(rename = "super")]
    pub super_cls: String,
    pub fields: Vec<FieldDeserialize>,
    pub methods: Vec<CodeDeserialize>
}

#[derive(Deserialize)]
pub struct CodeDeserialize {
    pub name: String,
    pub descriptor: String,
    pub code: Vec<String>
}

#[derive(Deserialize)]
pub struct FieldDeserialize {
    pub name: String,
    pub descriptor: String,
    pub value: String
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