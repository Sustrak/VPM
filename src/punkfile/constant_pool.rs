use punkfile::constant_items::ConstantItems;
use super::serde_json::Value;

#[derive(Default, Debug)]
pub struct ConstantPool {
    pub pool: Vec<ConstantItems>
}

impl ConstantPool {
    pub fn new(values: &Vec<Value>) -> ConstantPool {
        let mut cp: ConstantPool = Default::default();
        for item in values {
            let cnst = match item["tag"].as_u64().unwrap() {
                1 => ConstantItems::ConstantInfo(item["info"].as_str().unwrap().to_string()),
                3 => ConstantItems::ConstantInteger(item["value"].as_i64().unwrap()),
                7 => ConstantItems::ConstantClass(item["cls_index"].as_u64().unwrap() as usize),
                8 => ConstantItems::ConstantString(item["value"].as_str().unwrap().to_string()),
                9 => ConstantItems::ConstantField {
                        class: item["cls_index"].as_u64().unwrap() as usize,
                        name_type: item["name_type"].as_u64().unwrap() as usize,
                    },
                10 => ConstantItems::ConstantMethod {
                        class: item["cls_index"].as_u64().unwrap() as usize,
                        name_type: item["name_type"].as_u64().unwrap() as usize
                    },
                13 => ConstantItems::ConstantNameType {
                    desc: item["desc_index"].as_u64().unwrap() as usize,
                    name: item["name_index"].as_u64().unwrap() as usize
                },
                x => ConstantItems::ConstantUnimplemented(x)
            };
            cp.pool.push(cnst);
        }
        cp
    }
}