#![allow(dead_code)]
struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

// using math
struct DinnerPlates {
    capacity: usize,
    vals: Vec<Option<i32>>,
    // when push ,first check those empty slot
    empty_slots: BinaryHeap<Reverse<usize>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DinnerPlates {
    fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            vals: Vec::new(),
            empty_slots: BinaryHeap::new(),
        }
    }

    fn push(&mut self, val: i32) {
        while let Some(Reverse(pos)) = self.empty_slots.pop() {
            // must be a valid slot incase we pop
            if pos < self.vals.len() {
                self.vals[pos] = Some(val);
                return;
            }
        }

        self.vals.push(Some(val));
    }

    fn pop(&mut self) -> i32 {
        while let Some(val) = self.vals.pop() {
            if val.is_some() {
                return val.unwrap();
            }
        }

        -1
    }

    // index means chunk index
    fn pop_at_stack(&mut self, index: i32) -> i32 {
        let idx: usize = index as usize;
        if idx >= (self.vals.len() + self.capacity - 1) / self.capacity {
            return -1;
        }
        let start = idx * self.capacity;
        let end = self.vals.len().min(start + self.capacity);

        for i in (start..end).rev() {
            if let Some(val) = self.vals[i] {
                self.vals[i] = None;
                self.empty_slots.push(Reverse(i));

                return val;
            }
        }

        -1
    }
}

fn main() {}
