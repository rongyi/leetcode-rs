struct Solution;

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
                self.data.entry(start).and_modify(|v| *v -= 1);
                self.data.entry(end).and_modify(|v| *v += 1);
                return false;
            }
        }

        true
    }
}

mod gemini {
    struct MyCalendarTwo {
        bookings: Vec<(i32, i32)>,
        overlaps: Vec<(i32, i32)>,
    }

    impl MyCalendarTwo {
        fn new() -> Self {
            MyCalendarTwo {
                bookings: Vec::new(),
                overlaps: Vec::new(),
            }
        }

        fn book(&mut self, start: i32, end: i32) -> bool {
            // Step 1: Check if adding (start, end) causes a triple booking
            for &(s, e) in &self.overlaps {
                if start.max(s) < end.min(e) {
                    return false; // Overlaps with an existing double booking
                }
            }

            // Step 2: Record new double-booked regions created by this event
            for &(s, e) in &self.bookings {
                let overlap_start = start.max(s);
                let overlap_end = end.min(e);
                if overlap_start < overlap_end {
                    self.overlaps.push((overlap_start, overlap_end));
                }
            }

            // Step 3: Record the new single booking
            self.bookings.push((start, end));
            true
        }
    }
}

fn main() {}
