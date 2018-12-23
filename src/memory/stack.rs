#[derive(Clone)]
pub struct Stack<T> {
    stack: Vec<T>
}

impl<T> Stack<T> {

    #[inline]
    pub const fn new() -> Stack<T> {
        Stack {stack: Vec::new()}
    }

    pub fn push(&mut self, element: &T) {
        let mut v = vec![*element];
        self.stack.append(&mut v)
    }

    pub fn pop(&mut self) -> &T {
        match self.stack.first() {
            Some(x) => {
                let res = x.clone();
                self.stack.remove(0);
                x
            }
            None => panic!("Trying to pop an element from an empty stack")
        }
    }

    pub fn pop2(&mut self) -> (&T, &T) {

    }

    pub fn get(&self) -> &T {
        match self.stack.first() {
            Some(x) => x,
            None => panic!("Trying to obtain the first element of the stack when this is empty"),
        }
    }

    pub fn get_mut(&mut self) -> &mut T {
        match self.stack.first_mut() {
            Some(x) => x,
            None => panic!("Trying to obtain the first element of the stack when this is empty"),
        }
    }
}