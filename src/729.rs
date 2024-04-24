#![allow(dead_code)]

use std::collections::BTreeMap;

struct MyCalendar {
    booked: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    fn new() -> Self {
        Self {
            booked: BTreeMap::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        if let Some((_, &prev_end)) = self.booked.range(..end).next_back() {
            if prev_end > start {
                return false;
            }
        }

        self.booked.insert(start, end);

        true
    }
}

fn main() {}
