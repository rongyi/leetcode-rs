#![allow(dead_code)]

struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SeatManager {
    fn new(n: i32) -> Self {
        let mut heap = BinaryHeap::new();
        for i in 1..=n {
            heap.push(Reverse(i));
        }
        SeatManager { heap }
    }

    fn reserve(&mut self) -> i32 {
        let Reverse(seat) = self.heap.pop().unwrap();
        seat
    }

    fn unreserve(&mut self, seat_number: i32) {
        self.heap.push(Reverse(seat_number));
    }
}

struct SeatManager {
    heap: BinaryHeap<Reverse<i32>>,
}

fn main() {}
