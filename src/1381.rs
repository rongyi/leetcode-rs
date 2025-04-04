#![allow(dead_code)]

use std::collections::BTreeMap;

struct CustomStack {
    stack: Vec<i32>,
    max_size: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CustomStack {
    fn new(max_size: i32) -> Self {
        Self {
            stack: Vec::new(),
            max_size: max_size as usize,
        }
    }

    fn push(&mut self, x: i32) {
        if self.stack.len() < self.max_size {
            self.stack.push(x);
        }
    }

    fn pop(&mut self) -> i32 {
        if self.stack.len() > 0 {
            self.stack.pop().unwrap()
        } else {
            -1
        }
    }

    fn increment(&mut self, k: i32, val: i32) {
        let k = (k as usize).min(self.stack.len());
        for i in 0..k {
            self.stack[i] += val;
        }
    }
}

/**
 * Your CustomStack object will be instantiated and called as such:
 * let obj = CustomStack::new(maxSize);
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * obj.increment(k, val);
 */

fn main() {}
