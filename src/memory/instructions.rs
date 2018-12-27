use std::collections::HashMap;
use isa::bytecode::ByteCode;

#[derive(Default)]
pub struct Instructions {
    // Map where K -> ClassMethod | V -> position where code starts
    methods: HashMap<String, usize>,
    code: Vec<ByteCode>,
}

impl Instructions {
    pub fn new_method(&mut self, name: String, code: &mut Vec<ByteCode>) {
        self.methods.insert(name, self.code.len());
        self.code.append(code);
    }

    pub fn get_ins(&self, pc: usize) -> &ByteCode {
        match self.code.get(pc) {
            Some(i) => i,
            None => panic!("At pc = {} there's no instruction", pc)
        }
    }

    pub fn get_method_pc(&self, name: &str) -> usize {
        match self.methods.get(name) {
            Some(m) => m.clone(),
            None => panic!("The method {} doesn't exist", name)
        }
    }
}