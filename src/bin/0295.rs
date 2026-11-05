struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};
struct MedianFinder {
    smallers: BinaryHeap<i32>,
    largers: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        Self {
            smallers: BinaryHeap::new(),
            largers: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        // first sink in smallers
        self.smallers.push(num);
        // then pop bigger one to big club
        let v = self.smallers.pop().unwrap();
        self.largers.push(Reverse(v));
        // balance if needed
        if self.largers.len() > self.smallers.len() {
            let v = self.largers.pop().unwrap().0;
            self.smallers.push(v);
        }
        // so smaller is the one who can one more larger with bigger club
    }

    fn find_median(&self) -> f64 {
        if self.smallers.len() == self.largers.len() {
            let v1: f64 = self.smallers.peek().unwrap().to_owned() as f64;
            let v2: f64 = self.largers.peek().unwrap().0.to_owned() as f64;
            (v1 + v2) / 2.0
        } else {
            self.smallers.peek().unwrap().to_owned() as f64
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */

fn main() {}
