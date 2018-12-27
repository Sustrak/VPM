use std::collections::HashMap;
use memory::vpk_stack::Type;

pub struct Objects {
    objects: HashMap<String, Vec<Type>>
}

impl Objects {
    pub fn get_field(&self, object: String, index: usize) -> Type {
        match self.objects.get(object.as_str()) {
            Some(cls) => match cls.get(index) {
                Some(v) => return v.clone(),
                None => panic!("The field in {} doesn't exist", index)
            },
            None => panic!("The object doesn't exist")
        }
    }

    pub fn set_field(&mut self, object: String, index: usize, value: Type) {
        match self.objects.get_mut(object.as_str()) {
            Some(cls) => cls.insert(index, value),
            None => panic!("The object doesn't exist")
        }
    }

    pub fn new_object(&mut self, object: String, fields: Vec<Type>) {
        self.objects.insert(object, fields);
    }
}