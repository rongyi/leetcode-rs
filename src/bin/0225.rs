struct Solution;

use std::collections::VecDeque;
struct MyStack {
    q1: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    fn new() -> Self {
        Self {
            q1: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        let sz = self.q1.len();
        self.q1.push_back(x);
        for _ in 0..sz {
            let val = self.q1.pop_front().unwrap();
            self.q1.push_back(val);
        }
    }

    fn pop(&mut self) -> i32 {
        self.q1.pop_front().unwrap()
    }

    fn top(&mut self) -> i32 {
        *self.q1.front().unwrap()
    }

    fn empty(&self) -> bool {
        self.q1.is_empty()
    }
}

fn main() {
    let mut stk = MyStack::new();
    stk.push(1);
    stk.push(2);
    let cur = stk.pop();
    println!("{}", cur);
}
