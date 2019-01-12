#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Type {
    Integer(i32),
    String(String),
    Object(String),
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum RetType {
    Integer,
    String,
    Object,
    Void,
}

#[derive(Clone, Debug)]
pub struct Frame {
    local_vars: Vec<Type>,
    stack: Vec<Type>,
    ret_type: RetType
}

#[derive(Debug)]
pub struct StackVM {
    stack: Vec<Frame>,
    ret_addr: Vec<usize>,
    pc: usize,
}

impl Type {
    pub fn is_object(&self) -> bool {
        match self {
            &Type::Object(_) => true,
            _ => false,
        }
    }
}

impl RetType {
    pub fn get_type(s: &str) -> RetType {
        match s {
            "I" => RetType::Integer,
            "S" => RetType::String,
            "O" => RetType::Object,
            "V" => RetType::Void,
            _ => panic!("The type {} is not a valid return type", s)
        }
    }
}

impl Frame {
    pub fn new() -> Frame {
        Frame {
            local_vars: Vec::new(),
            stack: Vec::new(),
            /** Used to set the behaviour of the return function */
            ret_type: RetType::Void
        }
    }
    /**
    Will get the top value of the operator stack and store it in the local variable
    */
    pub fn store_var(&mut self, i: usize) {
        let index = i;
        if self.local_vars.len() < index {
            let res = index - self.local_vars.len();
            self.local_vars.reserve(res);
        }

        let top = match self.stack.pop() {
            Some(x) => x,
            None => panic!("There's no elements on the stack"),
        };
        self.local_vars.insert(i, top)
    }

    /**
    Will get the local variable in index i and push it at the top of the operator stack
    */
    pub fn load_var(&mut self, i: usize) {
        let index = i;
        let var = match self.local_vars.get(index) {
            Some(x) => x.clone(),
            None => panic!("There's no localVariable in {}", i)
        };

        self.stack.push(var)
    }
    /**
    Will push variables into the variables Vec, used for the methodcalls
    */
    pub fn push_var(&mut self, var: Type) {
        self.local_vars.push(var)
    }

    /**
    Will pop a value from the operator stack and return it
    */
    pub fn pop(&mut self) -> Type {
        match self.stack.pop() {
            Some(x) => x,
            None => panic!("No element to return")
        }
    }

    /**
    Will push the value t into the operator stack
    */
    pub fn push(&mut self, t: Type) {
        self.stack.push(t)
    }

    /**
    Will set the return type of the frame
    */
    pub fn set_ret_type(&mut self, t: RetType) {
        self.ret_type = t;
    }

    /**
    Will return the return type of the frame
    */
    pub fn get_ret_type(&self) -> RetType {
        self.ret_type.clone()
    }
}

impl StackVM {
    pub fn new() -> StackVM {
        StackVM {
            stack: Vec::new(),
            ret_addr: Vec::new(),
            pc: 0
        }
    }

    pub fn push_frame(&mut self, f: Frame) {
        self.stack.push(f)
    }

    pub fn pop_frame(&mut self) -> Frame {
        match self.stack.pop() {
            Some(f) => f,
            None => panic!("There are no more frames to pop")
        }
    }

    pub fn get_frame_mut(&mut self) -> &mut Frame {
        match self.stack.last_mut() {
            Some(x) => x,
            None => panic!("There are no frames to get")
        }
    }

    pub fn methodcall_pc(&mut self, new_pc: usize) {
        self.ret_addr.push(self.pc+1);
        self.pc = new_pc
    }

    pub fn ret_pc(&mut self) {
        match self.ret_addr.pop() {
            Some(x) => self.pc = x,
            None => panic!("There's no return address in the stack")
        }
    }

    pub fn inc_pc(&mut self) {
        self.pc += 1;
    }

    pub fn get_pc(&self) -> usize {
        self.pc
    }

    pub fn new_pc(&mut self, n_pc: usize) {
        self.pc = n_pc
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}
