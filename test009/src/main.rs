use std::mem;
struct MyQueue {
    s1: Vec<i32>,
    s2: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        Self {
            s1: Vec::new(),
            s2: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.s1.push(x)
    }

    fn pop(&mut self) -> i32 {
        if !self.s2.is_empty() {
            return self.s2.pop().unwrap();
        }

        let mut v: Vec<i32> = mem::take(&mut self.s1);
        v.reverse();
        self.s2 = v;
        assert!(!self.s2.is_empty());
        self.s2.pop().unwrap()
    }

    fn peek(&self) -> i32 {
        if !self.s2.is_empty() {
            return self.s2.last().unwrap().clone();
        }

        assert!(!self.s1.is_empty());
        self.s1.first().unwrap().clone()
    }

    fn empty(&mut self) -> bool {
        self.s1.is_empty() && self.s2.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_basic() {
        let mut q = MyQueue::new();
        q.push(1);
        q.push(2);
        assert_eq!(q.peek(), 1);
        assert_eq!(q.empty(), false);
        assert_eq!(q.pop(), 1);
        assert_eq!(q.pop(), 2);
        assert_eq!(q.empty(), true);
    }
}

fn main() {
    println!("Hello, world!");
}
