#![allow(dead_code)]

use std::collections::BTreeMap;

struct MyCalendarThree {
    data: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarThree {
    fn new() -> Self {
        Self {
            data: BTreeMap::new(),
        }
    }

    fn book(&mut self, start_time: i32, end_time: i32) -> i32 {
        *self.data.entry(start_time).or_insert(0) += 1;
        *self.data.entry(end_time).or_insert(0) -= 1;
        let mut ongoing = 0;
        let mut k = 0;
        for (_, &v) in self.data.iter() {
            ongoing += v;
            k = k.max(ongoing);
        }
        k
    }
}

/**
 * Your MyCalendarThree object will be instantiated and called as such:
 * let obj = MyCalendarThree::new();
 * let ret_1: i32 = obj.book(startTime, endTime);
 */

fn main() {}
