use crate::memory::vpk_stack::{StackVM, Frame, Type, RetType};
use crate::memory::objects::Objects;
use std::collections::HashMap;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum ByteCode {
    MUL,
    DIV,
    SUB,
    POP,
    IADD,
    SADD,
    //NULL
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
    NEW {class: String, name: String},
    GETFIELD { object: String, local: String },
    PUTFIELD { object: String, local: String },
    METHODCALL { class: String, method: String },
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
    let x = frame.pop();
    let y = frame.pop();

    match (x, y) {
        (Type::Integer(xx), Type::Integer(yy)) => {
            let t = Type::Integer(xx / yy);
            frame.push(t);
            Ok(())
        },
        _ => Err("Only the multiplication of integer is supported")
    }
}

pub fn sub(frame: &mut Frame) -> Result<(), &'static str> {
    let x = frame.pop();
    let y = frame.pop();

    match (x, y) {
        (Type::Integer(xx), Type::Integer(yy)) => {
            let t = Type::Integer(xx - yy);
            frame.push(t);
            Ok(())
        },
        _ => Err("Only the multiplication of integer is supported")
    }
}

pub fn pop(frame: &mut Frame) -> Result<(), &'static str> {
    frame.pop();
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
        _ => Err("Only the multiplication of integer is supported")
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
        _ => Err("Only the multiplication of integer is supported")
    }
}

pub fn print(frame: &mut Frame) -> Result<(), &'static str> {
    let x = frame.pop();

    match x {
        Type::Integer(x) => {println!("{}", x); Ok(())},
        Type::String(x) => {println!("{}", x); Ok(())},
        Type::Object(_) => {Err("Printing an object is not supported")},
    }
}

pub fn ret(stack: &mut StackVM, frame: &mut Frame) -> Result<bool, &'static str> {
    if !stack.is_empty() {
        let ret_type = frame.get_ret_type();
        if *ret_type != RetType::Void {
            let ret = frame.pop();
            let mut callee_frame = stack.get_frame_mut();
            callee_frame.push(ret)
        }
        stack.ret_pc();
        return Ok(false)
    }
    else { Ok(true) }

}

pub fn new(objects: &mut Objects, object: String, fields: HashMap<String, Type>) -> Result<(), &'static str> {
    objects.new_object(object, fields);
    Ok(())
}

//pub fn label(stack: &mut Frame) -> Result<(), &'static str> {}

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

pub fn cnst(stack: &mut Frame, con: Type) -> Result<(), &'static str> {
    stack.push(con);
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

pub fn getfield(stack: &mut Frame, objects: &Objects, class: &String, local: &String) -> Result<(), &'static str> {
    let field = objects.get_field(class, local);
    stack.push(field);
    Ok(())
}

pub fn putfield(stack: &mut Frame, objects: &mut Objects, class: &String, local: &String) -> Result<(), &'static str> {
    let field = stack.pop();
    objects.set_field(class, local, field);
    Ok(())
}

/// A new frame will be created and all the parameters will be stored in local variables in the order of the signature
/// *-------------*
/// *    STACK    *
/// *-------------*
/// *    arg0     *
/// *    arg1     *
/// *    ....     *
/// *    argN     *
/// *-------------*
///
pub fn methodcall(stack: &mut StackVM, signature: &String, new_pc: usize, mut frame: Frame) -> Result<(), &'static str> {
    let v: Vec<&str> = signature.split(|c| c == '(' || c == ')').collect();
    let n_args: usize = v[1].len();
    let mut new_frame: Frame = Frame::new();
    new_frame.set_ret_type(RetType::get_type(v[2]));
    for _ in 0..n_args {
        new_frame.push_var(frame.pop())
    }
    stack.push_frame(new_frame);
    stack.methodcall_pc(new_pc);

    Ok(())
}


