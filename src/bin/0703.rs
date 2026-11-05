#![allow(dead_code)]

use std::{cmp::Reverse, collections::BinaryHeap};

struct KthLargest {
    k: usize,
    nums: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, mut nums: Vec<i32>) -> Self {
        let mut ret = Self {
            k: k as usize,
            nums: BinaryHeap::new(),
        };

        for num in nums.into_iter() {
            ret.nums.push(Reverse(num));
            if ret.nums.len() > ret.k {
                ret.nums.pop();
            }
        }

        ret
    }

    fn add(&mut self, val: i32) -> i32 {
        self.nums.push(Reverse(val));
        if self.nums.len() > self.k {
            self.nums.pop();
        }
        let Reverse(val) = *self.nums.peek().unwrap();

        val
    }
}

fn main() {}
