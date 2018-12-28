use punkfile::constant_pool::ConstantPool;
use punkfile::code::Code;
use punkfile::deserializer::ClassDeserialize;
use punkfile::field::Field;

#[derive(Default, Debug)]
pub struct Class {
    pub constant_pool: ConstantPool,
    pub this: String,
    pub super_cls: String,
    pub fields: Vec<Field>,
    pub methods: Vec<Code>,
}

impl Class {
    pub fn new(cls: ClassDeserialize) -> Class {
        let mut c: Class = Default::default();
        c.constant_pool = ConstantPool::new(&cls.constant_pool);
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
}