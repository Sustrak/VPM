use memory::vpk_stack::Type;

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

pub fn mul(stack: &mut Vec<Type>) -> Result<(), &'static str> {
    let x = stack.pop().unwrap();
    let y = stack.pop().unwrap();

    match (x, y) {
        (Type::Integer(xx), Type::Integer(yy)) => {
            let t = Type::Integer(xx * yy);
            stack.push(t);
            Ok(())
        },
        _ => Err("Only the multiplication of integer is supported")
    }
}

pub fn div(stack: &mut Vec<Type>) -> Result<(), &'static str> {
    let x = stack.pop().unwrap();
    let y = stack.pop().unwrap();

    match (x, y) {
        (Type::Integer(xx), Type::Integer(yy)) => {
            let t = Type::Integer(xx / yy);
            stack.push(t);
            Ok(())
        },
        _ => Err("Only the multiplication of integer is supported")
    }
}

pub fn sub(stack: &mut Vec<Type>) -> Result<(), &'static str> {
    let x = stack.pop().unwrap();
    let y = stack.pop().unwrap();

    match (x, y) {
        (Type::Integer(xx), Type::Integer(yy)) => {
            let t = Type::Integer(xx - yy);
            stack.push(t);
            Ok(())
        },
        _ => Err("Only the multiplication of integer is supported")
    }
}

pub fn pop(stack: &mut Vec<Type>) -> Result<(), &'static str> {
    match stack.pop() {
        Some(_) => Ok(()),
        None => Err("There's no element to POP from the stack")
    }
}

pub fn iadd(stack: &mut Vec<Type>) -> Result<(), &'static str> {
    let x = stack.pop().unwrap();
    let y = stack.pop().unwrap();

    match (x, y) {
        (Type::Integer(xx), Type::Integer(yy)) => {
            let t = Type::Integer(xx + yy);
            stack.push(t);
            Ok(())
        },
        _ => Err("Only the multiplication of integer is supported")
    }
}

pub fn sadd(stack: &mut Vec<Type>) -> Result<(), &'static str> {
    let x = stack.pop().unwrap();
    let y = stack.pop().unwrap();

    match (x, y) {
        (Type::String(xx), Type::String(yy)) => {
            let t = Type::String(format!("{}{}", xx, yy));
            stack.push(t);
            Ok(())
        },
        _ => Err("Only the multiplication of integer is supported")
    }
}

pub fn print(stack: &mut Vec<Type>) -> Result<(), &'static str> {
    let x = stack.pop().unwrap();

    match x {
        Type::Integer(x) => {println!("{}", x); Ok(())},
        Type::String(x) => {println!("{}", x); Ok(())},
        Type::Object(_) => {Err("Printing an object is not supported")},
    }
}

pub fn ret(stack: &mut Vec<Type>) -> Result<(), &'static str> {}
pub fn new(stack: &mut Vec<Type>) -> Result<(), &'static str> {}
pub fn label(stack: &mut Vec<Type>) -> Result<(), &'static str> {}
pub fn goto(stack: &mut Vec<Type>) -> Result<(), &'static str> {}
pub fn load(stack: &mut Vec<Type>) -> Result<(), &'static str> {}
pub fn store(stack: &mut Vec<Type>) -> Result<(), &'static str> {}
pub fn cnst(stack: &mut Vec<Type>) -> Result<(), &'static str> {}
pub fn if_eq(stack: &mut Vec<Type>) -> Result<(), &'static str> {}
pub fn if_cmplt(stack: &mut Vec<Type>) -> Result<(), &'static str> {}
pub fn if_cmpeq(stack: &mut Vec<Type>) -> Result<(), &'static str> {}
pub fn getfield(stack: &mut Vec<Type>) -> Result<(), &'static str> {}
pub fn putfield(stack: &mut Vec<Type>) -> Result<(), &'static str> {}
pub fn methodcall(stack: &mut Vec<Type>) -> Result<(), &'static str> {}


