use crate::punkfile::class::Class;
use crate::punkfile::main::Main;
use crate::punkfile::deserializer::PunkFileJSON;

const MAGIC_NUMBER: &str =  "CAFECAFE";

#[derive(Default, Debug)]
pub struct PunkFile {
    pub classes: Vec<Class>,
    pub main: Main,
}

impl PunkFile {
    pub fn from_file(uri: &str) -> PunkFile {
        let pk_des: PunkFileJSON = PunkFileJSON::from_file(uri);
        let mut pk: PunkFile = Default::default();

        for cls in pk_des.classes {
            let c: Class = Class::new(cls);
            pk.classes.push(c);
        }

        pk.main = Main::new(pk_des.main_code);

        pk
    }

    pub fn find_class(&self, name: &str) -> Option<&Class> {
        self.classes.iter().find(|c| c.this == name.to_string())
    }
}