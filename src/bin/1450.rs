#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        let mut intervals: Vec<(i32, i32)> =
            start_time.into_iter().zip(end_time.into_iter()).collect();
        intervals.sort_unstable();

        let mut count = 0;
        for (start_time, end_time) in intervals.into_iter() {
            if start_time > query_time {
                break;
            }
            if end_time < query_time {
                continue;
            }
            count += 1;
        }

        count
    }
}

fn main() {}
