use crate::punkfile::code::Code;
use crate::punkfile::deserializer::ClassDeserialize;
use crate::punkfile::field::Field;

#[derive(Default, Debug)]
pub struct Class {
    pub this: String,
    pub super_cls: String,
    pub fields: Vec<Field>,
    pub methods: Vec<Code>,
}

impl Class {
    pub fn new(cls: ClassDeserialize) -> Class {
        let mut c: Class = Default::default();
        c.this = cls.this;
        c.super_cls = cls.super_cls;
        for field in cls.fields {
            c.fields.push(Field::new(field))
        }
        for method in cls.methods {
            c.methods.push(Code::new(method))
        }
        c
    }

    pub fn find_method(&self, method: &str) -> Option<&Code> {
        self.methods.iter().find(|m| m.name == method.to_string())
    }
}