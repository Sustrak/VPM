#[derive(Eq, PartialEq, Clone)]
pub enum Type {
    Integer(i32),
    String(String),
    Object(usize),
}

struct Frame {
    local_vars: Vec<Type>,
    stack: Vec<Type>,
}

struct StackVM {
    stack: Vec<Frame>
}

impl Frame {
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
    pub fn get_var(&mut self, i: usize) {
        let index = i - 1;
        let var = match self.local_vars.get(index) {
            Some(x) => x.clone(),
            None => panic!("There's no localVariable in {}", i)
        };

        self.stack.push(var)
    }
}

impl StackVM {
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
}
