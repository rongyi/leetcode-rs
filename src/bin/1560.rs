#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn most_visited(n: i32, rounds: Vec<i32>) -> Vec<i32> {
        let start = rounds[0];
        let end = *rounds.last().unwrap();

        let mut result = Vec::new();

        // start --------- end
        if start <= end {
            // Simple case: when start <= end, the most visited sectors are just [start, end]
            for i in start..=end {
                result.push(i);
            }
        } else {
            // Complex case: when start > end, the most visited sectors are [1, end] and [start, n]
            // 1 ----- end ----- start ----- n
            for i in 1..=end {
                result.push(i);
            }
            for i in start..=n {
                result.push(i);
            }
        }

        result
    }
}

fn main() {}
