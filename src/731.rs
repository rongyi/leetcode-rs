#![allow(dead_code)]

struct MyCalendar {
    data: Vec<(i32, i32)>,
}

impl MyCalendar {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
    pub fn book(&mut self, start: i32, end: i32) -> bool {
        for &(cur_start, cur_end) in self.data.iter() {
            if cur_start.max(start) < cur_end.min(end) {
                return false;
            }
        }
        self.data.push((start, end));

        true
    }
}

struct MyCalendarTwo {
    data: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarTwo {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        let mut non_collsion = MyCalendar::new();
        for &(cur_start, cur_end) in self.data.iter() {
            let cur = (cur_start.max(start), cur_end.min(end));

            if !non_collsion.book(cur.0, cur.1) {
                return false;
            }
        }

        self.data.push((start, end));

        true
    }
}

fn main() {}
