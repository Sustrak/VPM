use std::collections::HashMap;
use crate::isa::bytecode::ByteCode;

#[derive(Default)]
pub struct Instructions {
    // Map where K -> ClassMethod | V -> position where code starts
    pub methods: HashMap<String, usize>,
    pub labels: HashMap<String, usize>,
    pub code: Vec<ByteCode>,
}

impl Instructions {
    pub fn new_method(&mut self, name: String, code: &mut Vec<ByteCode>) {
        let method_pc = self.code.len();
        self.methods.insert(name, method_pc);
        self.get_labels(method_pc, code);
        self.code.append(code);
    }

    pub fn get_ins(&self, pc: usize) -> &ByteCode {
        match self.code.get(pc) {
            Some(i) => i,
            None => panic!("At pc = {} there's no instruction", pc)
        }
    }

    /// name should be CLASS/METHOD
    pub fn get_method_pc(&self, name: &str) -> Option<usize> {
        match self.methods.get(name) {
            Some(m) => Some(m.clone()),
            None => None
        }
    }

    fn get_labels(&mut self, base_pc: usize, code: &Vec<ByteCode>) {
        let mut pc = base_pc;
        for c in code {
            match c {
                ByteCode::LABEL(label) => {
                    self.labels.insert(label.clone(), pc);
                },
                _ => ()
            };
            pc += 1;
        }
    }

    pub fn get_label_pc(&self, label: &String) -> usize {
        match self.labels.get(label.as_str()) {
            Some(pc) => pc.clone(),
            None => panic!("The label {} no exist", label)
        }
    }
}