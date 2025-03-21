#![allow(dead_code)]

struct Solution;

use std::collections::VecDeque;
struct FrontMiddleBackQueue {
    data: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FrontMiddleBackQueue {
    fn new() -> Self {
        Self {
            data: VecDeque::new(),
        }
    }

    fn push_front(&mut self, val: i32) {
        self.data.push_front(val);
    }

    fn push_middle(&mut self, val: i32) {
        let sz = self.data.len();

        self.data.insert(sz / 2, val)
    }

    fn push_back(&mut self, val: i32) {
        self.data.push_back(val);
    }

    fn pop_front(&mut self) -> i32 {
        self.data.pop_front().unwrap_or(-1)
    }

    fn pop_middle(&mut self) -> i32 {
        let sz = self.data.len();

        self.data.remove((sz - 1) / 2).unwrap_or(-1)
    }

    fn pop_back(&mut self) -> i32 {
        self.data.pop_back().unwrap_or(-1)
    }
}

fn main() {}
