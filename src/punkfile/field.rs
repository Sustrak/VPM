use crate::punkfile::deserializer::FieldDeserialize;
use crate::memory::vpk_stack::Type;

#[derive(Default, Debug)]
pub struct Field {
    pub name: String,
    pub desc: String,
    pub value: String,
}

impl Field {
    pub fn new(fd: FieldDeserialize) -> Field {
        Field {
            name: fd.name,
            desc: fd.descriptor,
            value: fd.value
        }
    }

    pub fn get_type(&self) -> Type {
        match self.value.parse::<i32>() {
            Ok(i) => Type::Integer(i),
            Err(_) => Type::String(self.value.clone())
        }
    }
}