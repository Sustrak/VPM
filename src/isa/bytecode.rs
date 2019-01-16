use crate::memory::vpk_stack::{StackVM, Frame, Type, RetType};
use crate::memory::objects::Objects;
use std::collections::HashMap;
use crate::punkfile::punk_file::PunkFile;
use crate::memory::instructions::Instructions;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum ByteCode {
    MUL,
    DIV,
    SUB,
    POP,
    IADD,
    SADD,
    NULL,
    PRINT,
    RETURN,
    LABEL(String),
    GOTO(String),
    LOAD(usize),
    STORE(usize),
    CONST(String),
    IF_EQ(String),
    IF_CMPLT(String),
    IF_CMPEQ(String),
    NEW { class: String },
    GETFIELD { field: String },
    PUTFIELD { field: String },
    METHODCALL { method: String },
}

pub fn mul(frame: &mut Frame) -> Result<(), &'static str> {
    let x = frame.pop();
    let y = frame.pop();

    match (x, y) {
        (Type::Integer(xx), Type::Integer(yy)) => {
            let t = Type::Integer(xx * yy);
            frame.push(t);
            Ok(())
        },
        _ => Err("Only the multiplication of integer is supported")
    }
}

pub fn div(frame: &mut Frame) -> Result<(), &'static str> {
    let y = frame.pop();
    let x = frame.pop();

    match (x, y) {
        (Type::Integer(xx), Type::Integer(yy)) => {
            let t = Type::Integer(xx / yy);
            frame.push(t);
            Ok(())
        },
        _ => Err("Only the division of integer is supported")
    }
}

pub fn sub(frame: &mut Frame) -> Result<(), &'static str> {
    let y = frame.pop();
    let x = frame.pop();

    match (x, y) {
        (Type::Integer(xx), Type::Integer(yy)) => {
            let t = Type::Integer(xx - yy);
            frame.push(t);
            Ok(())
        },
        _ => Err("Only the subtraction of integer is supported")
    }
}

pub fn pop(frame: &mut Frame) -> Result<(), &'static str> {
    frame.pop();
    Ok(())
}

pub fn null(frame: &mut Frame) -> Result<(), &'static str> {
    frame.push(Type::Object("Null".to_string()));
    Ok(())
}

pub fn iadd(frame: &mut Frame) -> Result<(), &'static str> {
    let x = frame.pop();
    let y = frame.pop();

    match (x, y) {
        (Type::Integer(xx), Type::Integer(yy)) => {
            let t = Type::Integer(xx + yy);
            frame.push(t);
            Ok(())
        },
        _ => Err("Only the addition of integer is supported")
    }
}

pub fn sadd(frame: &mut Frame) -> Result<(), &'static str> {
    let y = frame.pop();
    let x = frame.pop();

    match (x, y) {
        (Type::String(xx), Type::String(yy)) => {
            let t = Type::String(format!("{}{}", xx, yy));
            frame.push(t);
            Ok(())
        },
        (Type::String(xx), Type::Integer(yy)) => {
            let t = Type::String(format!("{}{}", xx, yy));
            frame.push(t);
            Ok(())
        },
        (Type::Integer(xx), Type::Integer(yy)) => {
            let t = Type::String(format!("{}{}", xx, yy));
            frame.push(t);
            Ok(())
        },
        (Type::Integer(xx), Type::String(yy)) => {
            let t = Type::String(format!("{}{}", xx, yy));
            frame.push(t);
            Ok(())
        },
        _ => Err("Can not construct an string with an object or boolean")
    }
}

pub fn print(frame: &mut Frame) -> Result<(), &'static str> {
    let x = frame.pop();

    match x {
        Type::Integer(x) => {println!("{}", x); Ok(())},
        Type::String(x) => {println!("{}", x); Ok(())},
        Type::Boolean(b) => {println!("{}", b); Ok(())}
        _ => Err("Printing an object is not supported"),
    }
}

/**
Will return true if the execution of the program has to stop because there are no more Frames on the VMStack
in the other way will return false.
It will report an error if necessary
*/
pub fn ret(stack: &mut StackVM, frame: &mut Frame) -> Result<bool, &'static str> {
    if stack.is_empty() {
        Ok(true)
    }
    else {
        let ret_type = frame.get_ret_type();
        if ret_type != RetType::Void {
            let ret = frame.pop();
            let caller_frame = stack.get_frame_mut();
            caller_frame.push(ret)
        }
        stack.ret_pc();
        Ok(false)
    }
}

pub fn new(objects: &mut Objects, frame: &mut Frame, object: &String, fields: HashMap<String, Type>) -> Result<(), &'static str> {
    let reff = objects.new_object(object, fields);
    frame.push(Type::Object(reff));
    Ok(())
}

pub fn goto(stack: &mut StackVM, new_pc: usize) -> Result<(), &'static str> {
    stack.new_pc(new_pc);
    Ok(())
}

pub fn load(stack: &mut Frame, local: usize) -> Result<(), &'static str> {
    stack.load_var(local);
    Ok(())
}

pub fn store(stack: &mut Frame, local: usize) -> Result<(), &'static str> {
    stack.store_var(local);
    Ok(())
}

pub fn cnst(stack: &mut Frame, con: &String) -> Result<(), &'static str> {
    let t = match con.parse::<bool>() {
        Ok(b) => Type::Boolean(b),
        Err(_) => match con.parse::<i32>() {
            Ok(i) => Type::Integer(i),
            Err(_) => Type::String(con.clone()),
        }
    };
    stack.push(t);
    Ok(())
}

pub fn if_eq(frame: &mut Frame) -> Result<bool, &'static str> {
    let v = frame.pop();
    match v {
        Type::Integer(x) => {
            if x == 0 {
                return Ok(true)
            }
            Ok(false)
        }
        _ => Err("Can't compare when the value is not a integer")
    }
}

/// *-------------*
/// *    STACK    *
/// *-------------*
/// *     v1      *
/// *     v2      *
/// *-------------*
pub fn if_cmpeq(frame: &mut Frame) -> Result<bool, &'static str> {
    let v2 = frame.pop();
    let v1 = frame.pop();
    match (v1, v2) {
        (Type::Integer(x1), Type::Integer(x2)) => {
            if x1 == x2 {
                return Ok(true)
            }
            Ok(false)
        },
        (Type::Boolean(x1), Type::Boolean(x2)) => {
            if x1 == x2 {
                return Ok(true)
            }
            Ok(false)
        },
        _ => Err("Can't compare when the value is not a integer or boolean")
    }
}

/// *-------------*
/// *    STACK    *
/// *-------------*
/// *     v1      *
/// *     v2      *
/// *-------------*
pub fn if_cmplt(frame: &mut Frame) -> Result<bool, &'static str> {
    let v2 = frame.pop();
    let v1 = frame.pop();
    match (v1, v2) {
        (Type::Integer(x1), Type::Integer(x2)) => {
            if x1 < x2 {
                return Ok(true)
            }
            Ok(false)
        }
        _ => Err("Can't compare when the value is not a integer")
    }
}

/// *-------------*            *-------------*
/// *    STACK    *            *    STACK    *
/// *-------------*            *-------------*
/// *    obj.ref  *    --->    *    value    *
/// *-------------*            *-------------*
pub fn getfield(stack: &mut Frame, objects: &Objects, field: &String) -> Result<(), &'static str> {
    let object= match stack.pop() {
        Type::Object(obj) => obj,
        _ => return Err("A object reference was expected")
    };
    let field = objects.get_field(object, field);
    stack.push(field);
    Ok(())
}

/// *-------------*            *-------------*
/// *    STACK    *            *    STACK    *
/// *-------------*            *-------------*
/// *    obj.ref  *            *             *
/// *    value    *    --->    *             *
/// *-------------*            *-------------*
pub fn putfield(stack: &mut Frame, objects: &mut Objects, field: &String) -> Result<(), &'static str> {
    let value = stack.pop();
    let object= match stack.pop() {
        Type::Object(obj) => obj,
        _ => return Err("A object reference was expected")
    };
    objects.set_field(object, field, value);
    Ok(())
}

/// A new frame will be created and all the parameters will be stored in local variables in the order of the signature
/// *-------------*            *-------------*
/// *    STACK    *            *     VARS    *
/// *-------------*            *-------------*
/// *   obj.ref.  *            *   obj.ref.  * 0
/// *    arg0     *            *    arg0     * 1
/// *    arg1     *            *    arg1     * 2
/// *    ....     *    --->    *    ....     * ...
/// *    argN     *            *    argN     * N
/// *-------------*            *-------------*
pub fn methodcall(stack: &mut StackVM, punk_file: &PunkFile, ins: &Instructions, class: &str, method: &str) -> Result<(), &'static str> {
    let signature = match punk_file.find_class(class) {
        Some(c) => {
            match c.find_method(method) {
                Some(m) => m.desc.clone(),
                None => return Err("The method couldn't be found")
            }
        },
        None => return Err("The class couldn't be found")
    };
    let v: Vec<&str> = signature.split(|c| c == '(' || c == ')').collect();
    let n_args: usize = v[1].len();
    let mut new_frame: Frame = Frame::new();
    new_frame.set_ret_type(RetType::get_type(v[2]));
    let caller_frame = stack.get_frame_mut();
    let mut vars = Vec::new();
    for _ in 0..n_args {
        vars.push(caller_frame.pop())
    }
    let obj_ref = caller_frame.pop();
    if !obj_ref.is_object() {return Err("Expected a object reference. Stack malformed")}

    let mut cls_name: String;
    match obj_ref.clone() {
        Type::Object(name) => {
            cls_name = name;
            cls_name.retain(|c| !c.is_numeric())
        }
        _ => return Err("Expected a object reference. Stack malformed")
    };
    let mut new_pc = usize::max_value();
    while new_pc == usize::max_value() {
        //println!("DEBUG: new_pc {} | cls_name: {} | method: {}", new_pc, cls_name, method);
        match ins.get_method_pc(format!("{}/{}", cls_name, method).as_str()) {
            None => {
                match punk_file.find_class(cls_name.as_str()) {
                    None => return Err("No parent class found trying to get the pc method"),
                    Some(cls) => cls_name = cls.super_cls.clone(),
                }
            },
            Some(pc) => new_pc = pc,
        }
    }

    new_frame.push_var(obj_ref);
    for _ in 0..n_args {
        new_frame.push_var(vars.pop().unwrap())
    }
    stack.push_frame(new_frame);
    stack.methodcall_pc(new_pc);
    Ok(())
}


