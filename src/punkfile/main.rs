use punkfile::code::Code;
use super::serde_json::Value;

#[derive(Default)]
pub struct Main {
    fields: Vec<Code>,
    //code: Bytecode,
}

impl Main {
    pub fn set_info(fields: Value, code: Value) {

    }
}