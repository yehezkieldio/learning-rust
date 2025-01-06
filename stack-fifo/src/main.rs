struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack { stack: Vec::new() }
    }

    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    fn push(&mut self, value: T) {
        self.stack.push(value);
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    fn length(&self) -> usize {
        self.stack.len()
    }

    pub fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
}

fn main() {
    let mut stack: Stack<usize> = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    assert_eq!(stack.length(), 3);
    assert_eq!(stack.peek(), Some(&3));
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), None);
    assert_eq!(stack.is_empty(), true);
}

#[test]
fn test_stack() {
    let mut stack: Stack<usize> = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    assert_eq!(stack.length(), 3);
    assert_eq!(stack.peek(), Some(&3));
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), None);
    assert_eq!(stack.is_empty(), true);
}
