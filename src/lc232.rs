struct Solution;

struct MyQueue {
    insert_stack: Vec<i32>,
    pop_stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        Self {
            insert_stack: Vec::new(),
            pop_stack: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.insert_stack.push(x);
    }

    fn pop(&mut self) -> i32 {
        if self.pop_stack.last().is_none() {
            while let Some(val) = self.insert_stack.pop() {
                self.pop_stack.push(val);
            }
        }
        self.pop_stack.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        if self.pop_stack.last().is_none() {
            while let Some(val) = self.insert_stack.pop() {
                self.pop_stack.push(val);
            }
        }
        self.pop_stack.last().unwrap().to_owned()
    }

    fn empty(&self) -> bool {
        self.insert_stack.is_empty() && self.pop_stack.is_empty()
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

fn main() {}
