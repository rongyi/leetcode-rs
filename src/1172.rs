#![allow(dead_code)]

use std::{cmp::Reverse, collections::BinaryHeap};
struct Solution;

// calculate by math
struct DinnerPlates {
    cap: usize,
    // why reverse? make smaller index pop first
    empty_slot: BinaryHeap<Reverse<usize>>,
    vals: Vec<Option<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DinnerPlates {
    fn new(capacity: i32) -> Self {
        Self {
            cap: capacity as usize,
            empty_slot: BinaryHeap::new(),
            vals: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        // first check empty slot
        while let Some(Reverse(pos)) = self.empty_slot.pop() {
            // valid postion
            if pos < self.vals.len() {
                // update and go
                self.vals[pos] = Some(val);
                return;
            }
        }
        // or just normal push
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

    fn pop_at_stack(&mut self, index: i32) -> i32 {
        let index = index as usize;
        if index > self.vals.len() / self.cap {
            return -1;
        }
        let start = self.cap * index;
        let end = self.vals.len().min(start + self.cap);
        for i in (start..end).rev() {
            if let Some(num) = self.vals[i] {
                self.vals[i] = None;
                self.empty_slot.push(Reverse(i));

                return num;
            }
        }

        -1
    }
}

fn main() {}
