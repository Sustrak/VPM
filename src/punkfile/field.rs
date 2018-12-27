use punkfile::deserializer::FieldDeserialize;

#[derive(Default, Debug)]
pub struct Field {
    pub name: usize,
    pub desc: usize,
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
}