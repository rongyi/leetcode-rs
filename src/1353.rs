#![allow(dead_code)]
struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn max_events(mut events: Vec<Vec<i32>>) -> i32 {
        events.sort_unstable();

        let mut heap = BinaryHeap::new();
        let mut ret = 0;
        let mut i = 0;
        let mut day = 1;
        let sz = events.len();

        while i < sz || !heap.is_empty() {
            // Add all events that start on the current day to the heap
            while i < sz && events[i][0] == day {
                heap.push(Reverse(events[i][1]));
                i += 1;
            }
            // Remove events that are no longer available (end day < current day)
            while !heap.is_empty() && heap.peek().unwrap().0 < day {
                heap.pop();
            }

            // Attend the event with the earliest end day
            if !heap.is_empty() {
                heap.pop();
                ret += 1
            }

            day += 1
        }

        ret
    }
}

fn main() {}
