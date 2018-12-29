#[derive(Eq, PartialEq, Clone)]
pub enum Type {
    Integer(i32),
    String(String),
    Object(usize),
}

pub struct Frame {
    local_vars: Vec<Type>,
    stack: Vec<Type>,
}

pub struct StackVM {
    stack: Vec<Frame>,
    ret_addr: Vec<usize>,
    pc: usize
}

impl Frame {
    pub fn new() -> Frame {
        Frame {
            local_vars: Vec::new(),
            stack: Vec::new()
        }
    }
    /**
    Will get the top value of the stack and store it in the local variable
    */
    pub fn store_var(&mut self, i: usize) {
        let index = i - 1;
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
    Will get the local variable in index i and push it at the top of the stack
    */
    pub fn load_var(&mut self, i: usize) {
        let index = i - 1;
        let var = match self.local_vars.get(index) {
            Some(x) => x.clone(),
            None => panic!("There's no localVariable in {}", i)
        };

        self.stack.push(var)
    }

    pub fn push_var(&mut self, var: Type) {
        self.local_vars.push(var)
    }

    pub fn pop(&mut self) -> Type {
        match self.stack.pop() {
            Some(x) => x,
            None => panic!("No element to return")
        }
    }

    pub fn push(&mut self, t: Type) {
        self.stack.push(t)
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

    pub fn pop_frame(&mut self) {
        self.stack.pop();
    }

    pub fn get_frame(&self) -> &Frame {
        match self.stack.last() {
            Some(x) => x,
            None => panic!("There are no frames to get")
        }
    }

    pub fn get_frame_mut(&mut self) -> &mut Frame {
        match self.stack.last_mut() {
            Some(x) => x,
            None => panic!("There are no frames to get")
        }
    }

    pub fn methodcall_pc(&mut self, new_pc: usize) {
        self.ret_addr.push(self.pc);
        self.pc = new_pc
    }

    pub fn ret_pc(&mut self) {
        match self.ret_addr.pop() {
            Some(x) => self.pc = x,
            None => panic!("There's no return address in the stack")
        }
    }

    pub fn inc_pc(&mut self) -> usize {
        self.pc += 1;
        self.pc
    }

    pub fn new_pc(&mut self, n_pc: usize) {
        self.pc = n_pc
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}
