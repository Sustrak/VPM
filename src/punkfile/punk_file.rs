use punkfile::class::Class;
use punkfile::main::Main;
use punkfile::deserializer::PunkFileJSON;

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

    pub fn find_class(&self, name: &String) -> Result<&Class, &'static str> {
        let cls: Class;
        let mut find = false;
        for c in self.classes {
            if c.this == name {
                cls = c;
                find = true;
            }
        }
        if !find {
            Err(format!("Couldn't find the class {} in the constant pool", name).as_str())
        }
        else {
            Ok(&cls)
        }
    }
}