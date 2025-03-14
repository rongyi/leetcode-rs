#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn busiest_servers(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32> {
        use std::collections::{BTreeSet, BinaryHeap};

        let k = k as usize;
        let n = arrival.len();
        let mut busy: BinaryHeap<(i32, usize)> = BinaryHeap::new(); // min-heap of (end_time, server_id)
        let mut available: BTreeSet<usize> = (0..k).collect(); // available servers
        let mut requests_handled = vec![0; k]; // count requests handled by each server

        for i in 0..n {
            let start_time = arrival[i];
            let process_time = load[i];

            // Free up servers that have completed their tasks
            while !busy.is_empty() && -busy.peek().unwrap().0 <= start_time {
                let (_, server_id) = busy.pop().unwrap();
                available.insert(server_id);
            }

            // Skip if no servers available
            if available.is_empty() {
                continue;
            }

            // Find the server to assign the task
            let target_idx = i % k;
            let server_id = match available.range(target_idx..).next() {
                Some(&id) => id,
                None => *available.iter().next().unwrap(), // wrap around
            };

            // Assign the task to the server
            requests_handled[server_id] += 1;
            available.remove(&server_id);
            // Using negative time as BinaryHeap is max-heap by default
            // This allows us to simulate a min-heap for earliest completion time
            busy.push((-1 * (start_time + process_time), server_id));
        }

        // Find maximum requests handled
        let max_requests = *requests_handled.iter().max().unwrap();

        // Return server IDs with maximum requests
        requests_handled
            .iter()
            .enumerate()
            .filter(|&(_, &count)| count == max_requests)
            .map(|(id, _)| id as i32)
            .collect()
    }
}

fn main() {}
