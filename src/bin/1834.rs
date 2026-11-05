#![allow(dead_code)]

struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};
impl Solution {
    pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        // Create indexed tasks
        // (enque_time, process_time)
        let mut indexed_tasks: Vec<(i32, i32, i32)> = tasks
            .iter()
            .enumerate()
            .map(|(i, task)| (task[0], task[1], i as i32))
            .collect();
        // Sort by enqueue time
        indexed_tasks.sort_unstable();

        let mut result = Vec::with_capacity(tasks.len());
        let mut min_heap: BinaryHeap<(Reverse<i32>, Reverse<i32>)> = BinaryHeap::new();
        let mut current_time = indexed_tasks[0].0;
        let mut i = 0;

        while result.len() < tasks.len() {
            // Add all tasks that can be processed at current time to the heap
            while i < indexed_tasks.len() && indexed_tasks[i].0 <= current_time {
                let (_, processing_time, index) = indexed_tasks[i];
                min_heap.push((Reverse(processing_time), Reverse(index)));
                i += 1;
            }

            if !min_heap.is_empty() {
                // Process the task with shortest processing time
                let (Reverse(processing_time), Reverse(index)) = min_heap.pop().unwrap();
                result.push(index);
                current_time += processing_time;
            } else if i < indexed_tasks.len() {
                // If no tasks can be processed, jump to the next task's enqueue time
                current_time = indexed_tasks[i].0;
            }
        }

        result
    }
}

fn main() {}
