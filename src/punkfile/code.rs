use punkfile::deserializer::CodeDeserialize;

#[derive(Default)]
pub struct Code {
    name: usize,
    desc: usize,
    //code: Vec<Bytecode>,
}

impl Code {
    pub fn new(c: &CodeDeserialize) -> Code {}
}