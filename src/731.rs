#![allow(dead_code)]

use std::collections::BTreeMap;

struct MyCalendarTwo {
    data: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarTwo {
    fn new() -> Self {
        Self {
            data: BTreeMap::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        *self.data.entry(start).or_insert(0) += 1;
        *self.data.entry(end).or_insert(0) -= 1;

        let mut booked = 0;
        for (_, &v) in self.data.iter() {
            booked += v;
            if booked == 3 {
                *self.data.entry(start).or_insert(0) -= 1;
                *self.data.entry(end).or_insert(0) += 1;
                return false;
            }
        }

        true
    }
}

fn main() {}
