use std::convert::TryInto;

#[derive(Debug)]
pub struct Stack {
    stack: Vec<i32>,
}

impl Default for Stack {
    fn default() -> Self {
        Stack { stack: vec![] }
    }
}

impl Stack {
    pub fn pop(&mut self) -> i32 {
        let pop_result = self.stack.pop();
        if let Some(value) = pop_result {
            value
        } else {
            0
        }
    }

    pub fn push(&mut self, value: i32) {
        self.stack.push(value);
    }

    pub fn pop_char(&mut self) -> char {
        std::char::from_u32(self.pop().try_into().unwrap()).unwrap()
    }

    pub fn push_char(&mut self, value: char) {
        self.push(value as i32);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_stack() -> Stack {
        Stack::default()
    }

    #[test]
    fn push() {
        let mut stack = new_stack();
        stack.push(1);
        assert_eq!(1, stack.stack[0]);
    }

    #[test]
    fn push_char() {
        let mut stack = new_stack();
        stack.push_char('A');
        assert_eq!(65, stack.stack[0]);
    }

    #[test]
    fn pop() {
        let mut stack = new_stack();
        stack.push(9);
        assert_eq!(9, stack.pop());
    }

    #[test]
    fn pop_empty() {
        let mut stack = new_stack();
        assert_eq!(0, stack.pop());
    }

    #[test]
    fn pop_char() {
        let mut stack = new_stack();
        stack.push_char('A');
        assert_eq!('A', stack.pop_char());
    }

    #[test]
    fn pop_char_from_number() {
        let mut stack = new_stack();
        stack.push(65);
        assert_eq!('A', stack.pop_char());
    }
    #[test]
    fn pop_char_empty() {
        let mut stack = new_stack();
        assert_eq!(char::from(0), stack.pop_char());
    }
}
