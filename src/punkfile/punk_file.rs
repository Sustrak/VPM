use punkfile::class::Class;
use punkfile::main::Main;
use punkfile::deserializer::PunkFileJSON;

const MAGIC_NUMBER: &str =  "AAAA";

#[derive(Default)]
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

        let mut main: Main = Main::new(pk_des.main_code);

        pk
    }
}