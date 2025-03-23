#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        // Create a priority queue (max heap) to track our elements
        use std::collections::BinaryHeap;

        // Initialize our priority queue and track minimum value
        let mut pq = BinaryHeap::new();
        let mut min_val = i32::MAX;

        // First pass: make all odd numbers even by multiplying by 2
        // Push all values to priority queue
        for &num in &nums {
            let mut val = num;
            // If odd, multiply by 2 to make it even
            if val % 2 == 1 {
                val *= 2;
            }
            min_val = min_val.min(val);
            pq.push(val);
        }

        let mut min_deviation = i32::MAX;

        // Keep processing until we hit an odd maximum (can't reduce further)
        while let Some(max_val) = pq.pop() {
            // Update minimum deviation seen so far
            min_deviation = min_deviation.min(max_val - min_val);

            // If max value is odd, we can't reduce it further
            if max_val % 2 == 1 {
                break;
            }

            // Divide max value by 2 and push back to priority queue
            let new_val = max_val / 2;
            min_val = min_val.min(new_val);
            pq.push(new_val);
        }

        min_deviation
    }
}

fn main() {}
