pub struct Stack<T> {
    stack: Vec<T>
}

impl<T> Stack<T> {

    #[inline]
    pub const fn new() -> Stack<T> {
        Stack {stack: Vec::new()}
    }

    pub fn push(&mut self, element: T) {
        self.stack.append(element)
    }

    pub fn pop(&mut self) -> T {
        let ret = match self.stack.first() {
            Some(x) => {self.stack.remove(0); x}
            None => panic!("Trying to pop an element from an empty stack")
        };

        ret
    }

    pub fn get(&self) -> T {
        match self.stack.first() {
            Some(x) => x,
            None => panic!("Trying to obtain the first element of the stack when this is empty"),
        }
    }

    pub fn get_mut(&mut self) -> T {
        match self.stack.first_mut() {
            Some(x) => x,
            None => panic!("Trying to obtain the first element of the stack when this is empty"),
        }
    }
}