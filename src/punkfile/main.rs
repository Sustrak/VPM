use punkfile::code::Code;
use isa::bytecode::ByteCode;

#[derive(Default, Debug)]
pub struct Main {
    pub code: Vec<ByteCode>,
}

impl Main {
    pub fn new(code: Vec<String>) -> Main {
        let mut main: Main = Default::default();
        for ins in code {
            let mut split_inst = ins.split_whitespace();
            let instruction: ByteCode = match split_inst.next() {
                Some(v) => {
                    Code::parse_ins(&mut split_inst, v)
                },
                None => panic!("The code section is malformed"),
            };
            main.code.push(instruction);
        }
        main
    }
}