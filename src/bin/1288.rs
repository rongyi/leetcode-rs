#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn remove_covered_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        // sort by start
        intervals.sort_by(|x, y| {
            if x[0] == y[0] {
                // if same start, make larger range in header postion
                return y[1].cmp(&x[1]);
            }
            x[0].cmp(&y[0])
        });
        let mut end = intervals[0][1];
        let mut cnt = 1;

        for i in 1..intervals.len() {
            if intervals[i][1] > end {
                cnt += 1;
            }
            end = end.max(intervals[i][1]);
        }

        cnt
    }
}

fn main() {}
