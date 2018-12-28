#[macro_use]
extern crate serde_derive;

mod punkfile;
mod isa;
mod memory;

use punkfile::punk_file::PunkFile;
use memory::objects::Objects;
use memory::vpk_stack::StackVM;
use memory::instructions::Instructions;
use punkfile::constant_items::ConstantItems;
use memory::vpk_stack::Frame;
use isa::bytecode::ByteCode;
use isa::bytecode;
use punkfile::class::Class;
use memory::vpk_stack::Type;
use std::collections::HashMap;

fn main() {
    // Data structures
    let mut objects: Objects = Default::default();
    let mut stack: StackVM = StackVM::new();
    let mut instructions: Instructions = Default::default();
    let pk: PunkFile = PunkFile::from_file("scheme.json");

    // Initialize code structure
    for cls in pk.classes {
        let cls_name = cls.this;

        for m in cls.methods {
            let mthd_name = m.name;
            let name = format!("{}{}", cls_name, mthd_name);
            let mut code = m.code.to_owned();
            instructions.new_method(name, &mut code)
        }
    }
    let mut main_code = pk.main.code.to_owned();
    instructions.new_method(String::from("AppMain"), &mut main_code);

    // Initialize the stack with the main frame
    stack.push_frame(Frame::new());

    // Set the pc to the first instruction of the main code
    stack.new_pc(instructions.get_method_pc("AppMain") - 1);

    // Execute code
    loop {
        let pc= stack.inc_pc();
        let ins = instructions.get_ins(pc);
        let frame = stack.get_frame_mut();
        match ins {
            ByteCode::MUL => {
                match bytecode::mul(frame) {
                    Ok(()) => {},
                    Err(msg) => report_error(pc, msg)
                }
            },
            ByteCode::DIV => {
                match bytecode::div(frame) {
                    Ok(()) => {},
                    Err(msg) => report_error(pc, msg)
                }
            },
            ByteCode::SUB => {
                match bytecode::sub(frame) {
                    Ok(()) => {},
                    Err(msg) => report_error(pc, msg)
                }
            },
            ByteCode::POP => {
                match bytecode::pop(frame) {
                    Ok(()) => {},
                    Err(msg) => report_error(pc, msg)
                }
            },
            ByteCode::IADD => {
                match bytecode::iadd(frame) {
                    Ok(()) => {},
                    Err(msg) => report_error(pc, msg)
                }
            },
            ByteCode::SADD => {
                match bytecode::sadd(frame) {
                    Ok(()) => {},
                    Err(msg) => report_error(pc, msg)
                }
            },
            ByteCode::PRINT => {
                match bytecode::print(frame) {
                    Ok(()) => {},
                    Err(msg) => report_error(pc, msg)
                }
            },
            ByteCode::RETURN => {
                match bytecode::ret(&mut stack) {
                    Ok(()) => {},
                    Err(msg) => report_error(pc, msg)
                }
            },
            ByteCode::NEW {class, name} => {
                // Search the class in the constant pool
                let cls = match pk.find_class(class) {
                    Ok(c) => c,
                    Err(msg) => report_error(pc, msg)
                };

                let mut fields: HashMap<String, Type> = HashMap::new();
                // Check if has parent
                if cls.this != cls.super_cls {
                    let parent = match pk.find_class(&cls.super_cls) {
                        Ok(c) => c,
                        Err(msg) => report_error(pc, msg)
                    };
                    for f in parent.fields {
                        fields.insert(f.name, f.get_type())
                    }
                }
                for f in cls.fields {
                    fields.insert(f.name, f.get_type())
                }

                // Create object
                bytecode::new(&mut objects, name.clone(), fields);
            },
        }
    }

}

fn report_error(pc: usize, msg: &str) {
    panic!("At pc: {}, the execution throw an error: {}", pc, msg)
}
