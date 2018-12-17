use punkfile::constant_pool::ConstantPool;
use punkfile::code::Code;
use punkfile::deserializer::ClassDeserialize;

use super::serde_json::Value;

#[derive(Default)]
pub struct Class {
    constant_pool: ConstantPool,
    this: usize,
    super_cls: usize,
    fields: Vec<Code>,
    methods: Vec<Code>,
}

impl Class {
    pub fn new(cls: &ClassDeserialize) -> Class {
        let mut c: Class = Default::default();
        c.constant_pool = ConstantPool::new(&cls.constant_pool);
        c.this = cls.this;
        c.super_cls = cls.super_cls;
        for field in cls.fields {
            c.fields.push(Code::new(&field))
        }
        for method in cls.methods {
            c.methods.push(Code::new(&method))
        }
        c
    }
}