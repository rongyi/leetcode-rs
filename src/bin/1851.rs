#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_interval(intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut intervals = intervals;
        intervals.sort_by_key(|i| i[0]);

        let mut queries_with_index: Vec<_> =
            queries.iter().enumerate().map(|(i, &q)| (q, i)).collect();
        queries_with_index.sort_by_key(|&(q, _)| q);

        let mut result = vec![-1; queries.len()];
        let mut active_intervals = std::collections::BinaryHeap::new();
        let mut interval_index = 0;

        for (query, original_index) in queries_with_index {
            // Add all intervals starting before or at the query point
            while interval_index < intervals.len() && intervals[interval_index][0] <= query {
                let interval = &intervals[interval_index];
                let start = interval[0];
                let end = interval[1];

                // Only add intervals that contain the query point
                if end >= query {
                    // Add as a min-heap entry: (length, end)
                    active_intervals.push((-1 * (end - start + 1), end));
                }
                interval_index += 1;
            }

            // Remove intervals that end before the query point
            while !active_intervals.is_empty() && active_intervals.peek().unwrap().1 < query {
                active_intervals.pop();
            }

            // Get the smallest interval containing the query point
            if !active_intervals.is_empty() {
                result[original_index] = -active_intervals.peek().unwrap().0;
            }
        }

        result
    }
}

fn main() {}
