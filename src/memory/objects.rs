use std::collections::HashMap;
use memory::vpk_stack::Type;

#[derive(Default)]
pub struct Objects {
    objects: HashMap<String, HashMap<String, Type>>
}

impl Objects {
    pub fn get_field(&self, object: &String, field: &String) -> Type {
        match self.objects.get(object.as_str()) {
            Some(cls) => match cls.get(field.as_str()) {
                Some(v) => return v.clone(),
                None => panic!("The field in {} doesn't exist", field)
            },
            None => panic!("The object doesn't exist")
        }
    }

    pub fn set_field(&mut self, object: &String, field: &String, value: Type) {
        match self.objects.get_mut(object.as_str()) {
            Some(cls) => {
                match cls.get_mut(field.as_str()) {
                    Some(v) => *v = value,
                    None => panic!("Couldn't find the field {} in object {}", field, object)
                }
            }
            None => panic!("The object doesn't exist")
        }
    }

    pub fn new_object(&mut self, object: String, fields: HashMap<String, Type>) {
        self.objects.insert(object, fields);
    }
}