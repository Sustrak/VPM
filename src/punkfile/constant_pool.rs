use punkfile::constant_items::ConstantItems;
use super::serde_json::Value;
use std::fs::File;

#[derive(Default)]
pub struct ConstantPool {
    pool: Vec<ConstantItems>
}

impl ConstantPool {
    pub fn new(values: &Vec<Value>) -> ConstantPool {
        let mut cp: ConstantPool = Default::default();
        for item in values {
            let cnst = match item["tag"].as_u64()? {
                1 => ConstantItems::ConstantInfo(item["info"].as_str()?.to_string()),
                3 => ConstantItems::ConstantInteger(item["value"].as_i64()?),
                7 => ConstantItems::ConstantClass(item["cls_index"].as_u64()? as usize),
                8 => ConstantItems::ConstantString(item["value"].as_str()?.to_string()),
                9 => ConstantItems::ConstantField {
                        class: item["class"].as_u64()? as usize,
                        name_type: item["name_type"].as_u64()? as usize,
                    },
                10 => ConstantItems::ConstantMethod {
                        class: item["class"].as_u64()? as usize,
                        name_type: item["name_type"].as_u64()? as usize
                    },
                13 => ConstantItems::ConstantNameType {
                    desc: item["desc_index"].as_u64()? as usize,
                    name: item["name_index"].as_u64()? as usize
                }
            };
            cp.pool.push(cnst);
        }
        cp
    }
}