
struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn max_run_time(n: i32, batteries: Vec<i32>) -> i64 {
        let bts: Vec<i64> = batteries.into_iter().map(|i| i as i64).collect();
        let mut sum: i64 = bts.iter().sum();
        let mut n = n as i64;
        let mut q: BinaryHeap<i64> = bts.iter().copied().collect();

        while !q.is_empty() {
            if *q.peek().unwrap() > sum / n {
                n -= 1;
                sum -= *q.peek().unwrap();
                q.pop();
            } else {
                break;
            }
        }

        sum / n
    }
}

fn main() {}
