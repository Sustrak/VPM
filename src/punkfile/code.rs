use crate::punkfile::deserializer::CodeDeserialize;
use crate::isa::bytecode::ByteCode;
use std::str::SplitWhitespace;

#[derive(Default, Debug)]
pub struct Code {
    pub name: String,
    pub desc: String,
    pub code: Vec<ByteCode>,
}

impl Code {
    pub fn new(c: CodeDeserialize) -> Code {
        let mut code: Code = Default::default();
        code.name = c.name;
        code.desc = c.descriptor;

        for ins in c.code {
            let mut split_inst = ins.split_whitespace();
            let instruction: ByteCode = match split_inst.next() {
                Some(v) => {
                    Code::parse_ins(&mut split_inst, v)
                },
                None => panic!("The code section is malformed"),
            };
            code.code.push(instruction);
        }
        code
    }

    pub fn parse_ins(mut split_inst: &mut SplitWhitespace, v: &str) -> ByteCode {
        fn next(iter: &mut SplitWhitespace) -> String{
            iter.next().unwrap().to_string()
        }
        match v {
            "MUL" => ByteCode::MUL,
            "DIV" => ByteCode::DIV,
            "POP" => ByteCode::POP,
            "SUB" => ByteCode::SUB,
            "IADD" => ByteCode::IADD,
            "SADD" => ByteCode::SADD,
            "NULL" => ByteCode::NULL,
            "PRINT" => ByteCode::PRINT,
            "RETURN" => ByteCode::RETURN,
            "GOTO" => ByteCode::GOTO(next(&mut split_inst)),
            "LOAD" => ByteCode::LOAD(next(&mut split_inst).parse::<usize>().unwrap()),
            "CONST" => ByteCode::CONST({
                let mut s = split_inst.fold(String::new(), |acc, st| format!("{} {}", acc, st));
                s.remove(0);
                s
            }),
            "LABEL" => ByteCode::LABEL(next(&mut split_inst)),
            "STORE" => ByteCode::STORE(next(&mut split_inst).parse::<usize>().unwrap()),
            "IF_EQ" => ByteCode::IF_EQ(next(&mut split_inst)),
            "IF_CMPLT" => ByteCode::IF_CMPLT(next(&mut split_inst)),
            "IF_CMPEQ" => ByteCode::IF_CMPEQ(next(&mut split_inst)),
            "NEW" => ByteCode::NEW {
                class: next(&mut split_inst),
            },
            "GETFIELD" => ByteCode::GETFIELD {
                field: next(&mut split_inst)
            },
            "PUTFIELD" => ByteCode::PUTFIELD {
                field: next(&mut split_inst)
            },
            "METHODCALL" => ByteCode::METHODCALL {
                method: next(&mut split_inst)
            },
            x => panic!("The instruction {} not exist", x)
        }
    }
}