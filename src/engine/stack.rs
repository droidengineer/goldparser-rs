//! A simple stack.

use std::fmt;

/// A stack.
///
/// Supports only the most basic stack operations needed for the machine.
/// Implemending using a `Vec`.
///
/// ```
/// use engine::Stack;
/// let mut stack: Stack<usize> = Stack::new();
/// assert!(stack.is_empty());
///
/// stack.push(13);
/// assert!(!stack.is_empty());
///
/// let value = stack.pop();
/// assert_eq!(value, 13);
/// ```
#[derive(Debug, Default)]
pub struct Stack<T>(Vec<T>);

impl<T: fmt::Debug> Stack<T> {
    /// Create a new empty `Stack` and return it.
    pub fn new() -> Stack<T> {
        Stack(vec![])
    }

    /// Returns `true` if the stack contains no elements.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Push an element onto the top of the stack.
    pub fn push(&mut self, value: T) {
        self.0.push(value);
    }

    /// Pop the top element off the stack and return it.
    pub fn pop(&mut self) -> T {
        self.0.pop().expect("Unable to pop from empty stack!")
    }

    /// Take a sneaky look at the top element on the stack.
    pub fn peek(&self) -> Option<&T> {
        self.0.last()
        // let len = self.0.len();
        // if len == 0 {
        //     panic!("Cannot peek into empty stack!")
        // }
        // &self.0[len - 1]
    }

    /// Make a sneaky change to the top element on the stack.
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.0.last_mut()
        // let len = self.0.len();
        // if len == 0 {
        //     panic!("Cannot peek into empty stack!")
        // }
        // &mut self.0[len - 1]
    }

    pub fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
    pub fn len(&self) -> usize { self.0.len() }
}

impl<T> core::ops::Index<core::ops::Range<usize>> for Stack<T> {
    type Output = [T];

    fn index(&self, index: core::ops::Range<usize>) -> &Self::Output {
        self.0.index(index)
    }
}

#[derive(Debug)]
enum StackOp<T> {
    Push(T),
    Pop(T),
}







#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let stack: Stack<usize> = Stack::new();
        assert!(stack.is_empty());
    }

    #[test]
    fn push() {
        let mut stack: Stack<usize> = Stack::new();
        stack.push(13);
        assert!(!stack.is_empty());
    }

    #[test]
    fn pop() {
        let mut stack: Stack<usize> = Stack::new();
        stack.push(13);
        let value = stack.pop();
        assert_eq!(value, 13);
    }

    #[test]
    #[should_panic(expected = "empty stack")]
    fn empty_pop() {
        let mut stack: Stack<usize> = Stack::new();
        stack.pop();
    }

    #[test]
    fn peek() {
        let mut stack: Stack<usize> = Stack::new();
        stack.push(13);
        assert_eq!(*stack.peek().unwrap(), 13)
    }

    #[test]
    #[should_panic(expected = "empty stack")]
    fn empty_peek() {
        let stack: Stack<usize> = Stack::new();
        stack.peek();
    }
}
