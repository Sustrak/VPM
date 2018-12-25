use memory::vpk_stack::{StackVM, Frame, Type};
use memory::objects::Objects;

#[allow(non_camel_case_types)]
#[derive(Debug)]
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
    NEW(String),
    LABEL(String),
    GOTO(String),
    LOAD(String),
    STORE(String),
    CONST(String),
    IF_EQ(String),
    IF_CMPLT(String),
    IF_CMPEQ(String),
    GETFIELD { class: String, local: String },
    PUTFIELD { class: String, local: String },
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
    match frame.pop() {
        Some(_) => Ok(()),
        None => Err("There's no element to POP from the stack")
    }
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
    let x = frame.pop();
    let y = frame.pop();

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

pub fn ret(stack: &mut StackVM) -> Result<(), &'static str> {
    let result = stack.get_frame_mut().pop();
    // Delete the function frame
    stack.pop_frame();
    // Push the return to the top of the stack of the callee
    stack.get_frame_mut().push(result);
    // Set the new pc
    stack.ret_pc();
    Ok(())

}

pub fn new(stack: &mut Frame) -> Result<(), &'static str> {}
pub fn label(stack: &mut Frame) -> Result<(), &'static str> {}
pub fn goto(stack: &mut Frame) -> Result<(), &'static str> {}

pub fn load(stack: &mut Frame, local: usize) -> Result<(), &'static str> {
    stack.store_var(local);
    Ok(())
}

pub fn store(stack: &mut Frame, local: usize) -> Result<(), &'static str> {
    stack.load_var(local);
    Ok(())
}

pub fn cnst(stack: &mut Frame) -> Result<(), &'static str> {}
pub fn if_eq(stack: &mut Frame) -> Result<(), &'static str> {}
pub fn if_cmplt(stack: &mut Frame) -> Result<(), &'static str> {}
pub fn if_cmpeq(stack: &mut Frame) -> Result<(), &'static str> {}

pub fn getfield(stack: &mut Frame, objects: &Objects, class: String, local: usize) -> Result<(), &'static str> {
    let field = objects.get_field(class, local);
    stack.push(field);
    Ok(())
}

pub fn putfield(stack: &mut Frame, objects: &mut Objects, class: String, local: usize) -> Result<(), &'static str> {
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
pub fn methodcall(stack: &mut StackVM, signature: String, new_pc: usize) -> Result<(), &'static str> {
    let n_args: usize = 3;
    let frame = stack.get_frame_mut();
    let mut new_frame: Frame = Frame::new();

    for _ in 0..n_args {
        new_frame.push_var(frame.pop())
    }

    stack.push_frame(new_frame);


    Ok(())
}


