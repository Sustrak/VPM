use punkfile::deserializer::FieldDeserialize;
use memory::vpk_stack::Type;

#[derive(Default, Debug)]
pub struct Field {
    pub name: String,
    pub desc: String,
    pub value: String,
}

impl Field {
    pub fn new(fd: FieldDeserialize) -> Field {
        Field {
            name: fd.name_index,
            desc: fd.descriptor_index,
            value: fd.value
        }
    }

    pub fn get_type(&self) -> Type {
        match self.value.parse::<i32>() {
            Some(i) => Type::Integer(i),
            Err(_) => Type::String(self.value.clone())
        }
    }
}