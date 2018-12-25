use std::collections::HashMap;
use memory::vpk_stack::Type;

pub struct Objects {
    objects: HashMap<String, Vec<Type>>
}

impl Objects {
    pub fn get_field(&self, class: String, index: usize) -> Type {
        match self.objects.get(class.as_str()) {
            Some(cls) => match cls.get(index) {
                Some(v) => v,
                None => panic!("The field in {} doesn't exist", index)
            },
            None => panic!("The class doesn't exist")
        }
    }

    pub fn set_field(&mut self, class: String, index: usize, value: Type) {
        match self.objects.get_mut(class.as_str()) {
            Some(cls) => cls.insert(index, value),
            None => panic!("The class doesn't exist")
        }
    }
}