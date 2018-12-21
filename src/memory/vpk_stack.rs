use memory::stack::Stack;

pub enum Type {
    Integer(i32),
    String(String)
}

#[derive(Default)]
struct Frame {
    local_vars: Vec<Type>,
    stack: Stack<Type>,
}

#[derive(Default)]
struct StackVM {
    stack: Stack<Frame>
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

        let top = self.stack.pop();
        self.local_vars.insert(i, top)
    }

    /**
    Will get the local variable in index i and push it at the top of the stack
    */
    pub fn get_var(&mut self, i: usize) {
        let index = i - 1;
        let var = match self.local_vars.get(index) {
            Some(x) => x,
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

    pub fn get_frame(&self) -> Frame {
        self.stack.get()
    }

    pub fn get_frame_mut(&mut self) -> Frame {
        self.stack.get_mut()
    }
}