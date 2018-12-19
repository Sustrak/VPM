#[allow(non_camel_case_types)]
pub enum ByteCode {
    MUL,
    DIV,
    POP,
    SUB,
    IADD,
    SADD,
    RETURN,
    //NULL
    PRINT,
    NEW(String),
    //LABEL
    GOTO(String),
    LOAD(String),
    STORE(String),
    CONST(String),
    IF_EQ(String),
    IF_CMPLT(String),
    IF_CMPEQ(String),
    GETFIELD {class: String, local: String},
    PUTFIELD {class: String, local: String},
    METHODCALL {class: String, method: String},
}