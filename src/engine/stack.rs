//! A simple stack w/ stack rewind functionality.
//! 
//! This is a barebones stack implementation based on Vec<T>.
//! Stack rewind code from pest-parser by Dragoș Tiselice 

use super::alloc::vec;
use super::alloc::vec::Vec;
use core::ops::{Index,Range};

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
pub struct Stack<T: Clone> {
    ops: Vec<StackOp<T>>,
    stack: Vec<T>,
    snapshots: Vec<usize>,
}

impl<T: Clone> Stack<T> {
    /// Create a new empty `Stack` and return it.
    pub fn new() -> Self {
        Stack {        
            ops: vec![],
            stack: vec![],
            snapshots: vec![],
        }
    }

    /// Returns `true` if the stack contains no elements.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    /// Push an element onto the top of the stack.
    pub fn push(&mut self, value: T) {
        self.ops.push(StackOp::Push(value.clone()));
        self.stack.push(value);
    }

    /// Pop the top element off the stack and return it.
    pub fn pop(&mut self) -> Option<T> {
        let popped = self.stack.pop();
        if let Some(ref val) = popped {
            self.ops.push(StackOp::Pop(val.clone()));
        }
        popped
    }

    /// Take a sneaky look at the top element on the stack.
    pub fn peek(&self) -> Option<&T> {
        self.stack.last()
    }

    /// Make a sneaky change to the top element on the stack.
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.stack.last_mut()
    }

    pub fn as_slice(&self) -> &[T] {
        self.stack.as_slice()
    }

    // Stack Rewind [#2]
    /// Rewinds the `Stack` to the most recent `snapshot()`. If no `snapshot()` has been taken,
    /// this method returns the stack to its initial state.
    pub fn restore(&mut self) {
        match self.snapshots.pop() {
            Some(ops_index) => {
                self.rewind_to(ops_index);
                self.ops.truncate(ops_index);
            },
            None => {
                self.clear();
            }
        }
    }

    /// Rewind the stack to a particular index
    fn rewind_to(&mut self, index: usize) {
        let ops_to_rewind = &self.ops[index..];
        for op in ops_to_rewind.iter().rev() {
            match *op {
                StackOp::Push(_) => {
                    self.stack.pop();
                },
                StackOp::Pop(ref elem) => {
                    self.stack.push(elem.clone());
                }
            }
        }
    }

    /// Takes a snapshot of the current `Stack`
    pub fn snapshot(&mut self) {
        self.snapshots.push(self.ops.len());
    }

    /// The parsing after the last snapshot was successful so clear it
    pub fn clear_snapshot(&mut self) {
        self.snapshots.pop();
    }

    pub fn clear(&mut self) {
        self.ops.clear();
        self.stack.clear();
        self.snapshots.clear();
    }
    pub fn len(&self) -> usize { self.stack.len() }
}


impl<T: Clone> Index<Range<usize>> for Stack<T> {
    type Output = [T];

    fn index(&self, range: Range<usize>) -> &Self::Output {
        self.stack.index(range)
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

    // iss2 [#2]
    #[test]
    fn snapshot_with_empty() {
        let mut stack = Stack::new();

        stack.snapshot();
        // []
        assert!(stack.is_empty());
        // [0]
        stack.push(0);
        stack.restore();
        assert!(stack.is_empty());
    }

    #[test]
    fn snapshot_twice() {
        let mut stack = Stack::new();

        stack.push(0);

        stack.snapshot();
        stack.snapshot();
        stack.restore();
        stack.restore();

        assert_eq!(stack[0..stack.len()], [0]);
    }

    #[test]
    fn stack_ops() {
        let mut stack = Stack::new();

        // []
        assert!(stack.is_empty());
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.pop(), None);

        // [0]
        stack.push(0);
        assert!(!stack.is_empty());
        assert_eq!(stack.peek(), Some(&0));

        // [0, 1]
        stack.push(1);
        assert!(!stack.is_empty());
        assert_eq!(stack.peek(), Some(&1));

        // [0]
        assert_eq!(stack.pop(), Some(1));
        assert!(!stack.is_empty());
        assert_eq!(stack.peek(), Some(&0));

        // [0, 2]
        stack.push(2);
        assert!(!stack.is_empty());
        assert_eq!(stack.peek(), Some(&2));

        // [0, 2, 3]
        stack.push(3);
        assert!(!stack.is_empty());
        assert_eq!(stack.peek(), Some(&3));

        // Take a snapshot of the current stack
        // [0, 2, 3]
        stack.snapshot();

        // [0, 2]
        assert_eq!(stack.pop(), Some(3));
        assert!(!stack.is_empty());
        assert_eq!(stack.peek(), Some(&2));

        // Take a snapshot of the current stack
        // [0, 2]
        stack.snapshot();

        // [0]
        assert_eq!(stack.pop(), Some(2));
        assert!(!stack.is_empty());
        assert_eq!(stack.peek(), Some(&0));

        // []
        assert_eq!(stack.pop(), Some(0));
        assert!(stack.is_empty());

        // Test backtracking
        // [0, 2]
        stack.restore();
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(0));
        assert_eq!(stack.pop(), None);

        // Test backtracking
        // [0, 2, 3]
        stack.restore();
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(0));
        assert_eq!(stack.pop(), None);
    }


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
        assert_eq!(value.unwrap(), 13);
    }

    #[test]
    #[should_panic(expected = "empty stack")]
    fn empty_pop() {
        let mut stack: Stack<usize> = Stack::new();
        stack.pop().expect("empty stack");
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
