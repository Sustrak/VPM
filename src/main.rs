#[macro_use]
extern crate serde_derive;

mod punkfile;
mod isa;
mod memory;

use crate::punkfile::punk_file::PunkFile;
use crate::memory::objects::Objects;
use crate::memory::vpk_stack::{StackVM, Frame, Type};
use crate::memory::instructions::Instructions;
use crate::isa::bytecode::ByteCode;
use crate::isa::bytecode;
use std::collections::HashMap;
use std::env::args;

fn main() {
    // Data structures
    let mut objects: Objects = Objects::new();
    let mut stack: StackVM = StackVM::new();
    let mut instructions: Instructions = Default::default();
    let file_path = match args().nth(1) {
        Some(path) => path,
        None => panic!("Usage: vpk <some_file.json>")
    };
    let pk: PunkFile = PunkFile::from_file(file_path.as_str());

    // Initialize code structure
    for cls in pk.classes.iter() {
        let cls_name = &cls.this;

        for m in cls.methods.iter() {
            let mthd_name = &m.name;
            let name = format!("{}/{}", cls_name, mthd_name);
            let mut code = m.code.to_owned();
            instructions.new_method(name, &mut code)
        }
    }
    let mut main_code = pk.main.code.to_owned();
    instructions.new_method(String::from("AppMain"), &mut main_code);

    // Initialize the stack with the main frame
    stack.push_frame(Frame::new());

    // Set the pc to the first instruction of the main code
    stack.new_pc(instructions.get_method_pc("AppMain"));

    println!("DEBUG: {:?}", instructions.methods);
    println!("DEBUG: {:?}", instructions.code);

    // Execute code
    let mut pc: usize;
    let mut ins: &ByteCode;
    let mut frame = stack.pop_frame();
    loop {
        pc = stack.get_pc();
        ins = instructions.get_ins(pc);
        println!("DEBUG: pc {} ins {:?}", pc, ins);

        match ins {
            ByteCode::MUL => {
                match bytecode::mul(&mut frame) {
                    Ok(()) => {},
                    Err(msg) => report_error(pc, msg)
                }
            },
            ByteCode::DIV => {
                match bytecode::div(&mut frame) {
                    Ok(()) => {},
                    Err(msg) => report_error(pc, msg)
                }
            },
            ByteCode::SUB => {
                match bytecode::sub(&mut frame) {
                    Ok(()) => {},
                    Err(msg) => report_error(pc, msg)
                }
            },
            ByteCode::POP => {
                match bytecode::pop(&mut frame) {
                    Ok(()) => {},
                    Err(msg) => report_error(pc, msg)
                }
            },
            ByteCode::IADD => {
                match bytecode::iadd(&mut frame) {
                    Ok(()) => {},
                    Err(msg) => report_error(pc, msg)
                }
            },
            ByteCode::SADD => {
                match bytecode::sadd(&mut frame) {
                    Ok(()) => {},
                    Err(msg) => report_error(pc, msg)
                }
            },
            ByteCode::NULL => {
                match bytecode::null(&mut frame) {
                    Ok(()) => {},
                    Err(msg) => report_error(pc, msg)
                }
            }
            ByteCode::PRINT => {
                match bytecode::print(&mut frame) {
                    Ok(()) => {},
                    Err(msg) => report_error(pc, msg)
                }
            },
            ByteCode::RETURN => {
                match bytecode::ret(&mut stack, &mut frame) {
                    Ok(b) => {
                        if b {
                            // If there's no more frames the execution of the program should be done
                            break;
                        }
                    },
                    Err(msg) => report_error(pc, msg)
                }
            },
            ByteCode::NEW {class} => {
                // Search the class in the constant pool
                let cls = match pk.find_class(class) {
                    Some(c) => c,
                    None => report_error(pc, format!("The class {} couldn't be find", class).as_str())
                };

                let mut fields: HashMap<String, Type> = HashMap::new();
                // Check if has parent
                if cls.super_cls != "" {
                    let parent = match pk.find_class(&cls.super_cls) {
                        Some(c) => c,
                        None => report_error(pc, format!("The parent class {} of class {} couldn't be find", cls.super_cls ,class).as_str())
                    };
                    for f in parent.fields.iter() {
                        fields.insert(f.name.clone(), f.get_type());
                    }
                }
                for f in cls.fields.iter() {
                    fields.insert(f.name.clone(), f.get_type());
                }

                // Create object
                match bytecode::new(&mut objects, &mut frame, class, fields){
                    Ok(()) => {},
                    Err(msg) => report_error(pc, msg)
                }

                // Call constructor
                let reference = frame.pop();
                frame.push(reference.clone());
                frame.push(reference);
                stack.push_frame(frame.clone());
                let constructor_pc = instructions.get_method_pc(format!("{}/{}", class, "constructor").as_str());
                match bytecode::methodcall(&mut stack, &pk, constructor_pc, class, "constructor") {
                    Ok(()) => {},
                    Err(msg) => report_error(pc, msg),
                }
            },
            ByteCode::GOTO(label) => {
                let label_pc = instructions.get_label_pc(label);
                match bytecode::goto(&mut stack, label_pc) {
                    Ok(()) => {},
                    Err(msg) => report_error(pc, msg)
                }
            },
            ByteCode::LOAD(var) => {
                match bytecode::load(&mut frame, var.clone()) {
                    Ok(()) => {},
                    Err(msg) => report_error(pc, msg)
                }
            },
            ByteCode::STORE(var) => {
                match bytecode::store(&mut frame, var.clone()) {
                    Ok(()) => {},
                    Err(msg) => report_error(pc, msg)
                }
            },
            ByteCode::CONST(cst) => {
                match bytecode::cnst(&mut frame, cst) {
                    Ok(()) => {},
                    Err(msg) => report_error(pc, msg)
                }
            },
            ByteCode::IF_EQ(label) => {
                let label_pc = instructions.get_label_pc(label);
                match bytecode::if_eq(&mut frame) {
                    Ok(b) => {
                        if b {
                            stack.new_pc(label_pc)
                        }
                        else {
                            stack.inc_pc();
                        }
                    },
                    Err(msg) => report_error(pc, msg)
                }
            },
            ByteCode::IF_CMPLT(label) => {
                let label_pc = instructions.get_label_pc(label);
                match bytecode::if_cmplt(&mut frame) {
                    Ok(b) => {
                        if b {
                            stack.new_pc(label_pc)
                        }
                        else {
                            stack.inc_pc();
                        }
                    },
                    Err(msg) => report_error(pc, msg)
                }
            },
            ByteCode::IF_CMPEQ(label) => {
                let label_pc = instructions.get_label_pc(label);
                match bytecode::if_cmpeq(&mut frame) {
                    Ok(b) => {
                        if b {
                            stack.new_pc(label_pc)
                        }
                        else {
                            stack.inc_pc();
                        }
                    },
                    Err(msg) => report_error(pc, msg)
                }
            },
            ByteCode::GETFIELD {field} => {
                match bytecode::getfield(&mut frame, &objects, field) {
                    Ok(()) => {},
                    Err(msg) => report_error(pc, msg)
                }
            },
            ByteCode::PUTFIELD {field} => {
                match bytecode::putfield(&mut frame, &mut objects, field) {
                    Ok(()) => {},
                    Err(msg) => report_error(pc, msg)
                }
            },
            ByteCode::METHODCALL {method} => {
                stack.push_frame(frame.clone());
                let method_pc = instructions.get_method_pc(method.as_str());
                let v: Vec<&str> = method.split('/').collect();
                let class = v[0];
                let method_name = v[1];
                match bytecode::methodcall(&mut stack, &pk, method_pc, class, method_name) {
                    Ok(()) => {},
                    Err(msg) => report_error(pc, msg)
                }
            },
            _ => ()
        };

        match ins {
            ByteCode::GOTO(_) | ByteCode::IF_EQ(_) | ByteCode::IF_CMPEQ(_) | ByteCode::IF_CMPLT(_) | ByteCode::METHODCALL {method: _} | ByteCode::RETURN | ByteCode::NEW {class: _} => (),
            _ => {
                stack.inc_pc();
            }
        };

        match ins {
            ByteCode::METHODCALL {method: _} | ByteCode::RETURN | ByteCode::NEW {class: _} => {
                frame = stack.pop_frame()
            },
            _ => ()
        };

        //println!("DEBUG: {:#?}", frame)

    }

}

fn report_error(pc: usize, msg: &str) -> ! {
    panic!("At pc: {}, the execution throw an error: {}", pc, msg)
}
