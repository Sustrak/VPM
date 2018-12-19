use punkfile::class::Class;
use punkfile::main::Main;
use punkfile::deserializer::PunkFileJSON;

const MAGIC_NUMBER: &str =  "CAFECAFE";

#[derive(Default, Debug)]
pub struct PunkFile {
    classes: Vec<Class>,
    main: Main,
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
}