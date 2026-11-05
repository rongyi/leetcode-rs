#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let n = apples.len();
        let mut heap = BinaryHeap::new();
        let mut eaten = 0;
        let mut day = 0;

        while day < n || !heap.is_empty() {
            // Add new apples to the heap
            if day < n && apples[day] > 0 {
                // Push (expiry day, count) with the earliest expiry first
                heap.push(Reverse((day + days[day] as usize - 1, apples[day])));
            }

            // Remove expired apples
            while !heap.is_empty() {
                let Reverse((cur_day, _)) = *heap.peek().unwrap();
                if cur_day < day {
                    heap.pop();
                } else {
                    break;
                }
            }

            // Eat an apple if available
            if !heap.is_empty() {
                let Reverse((expiry, count)) = heap.pop().unwrap();
                eaten += 1;

                if count > 1 {
                    heap.push(Reverse((expiry, count - 1)));
                }
            }

            day += 1;
        }

        eaten
    }
}

fn main() {}
