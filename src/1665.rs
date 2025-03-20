#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn minimum_effort(mut tasks: Vec<Vec<i32>>) -> i32 {
        // We need to sort tasks by difference between actual and minimum energy
        tasks.sort_by(|a, b| (b[1] - b[0]).cmp(&(a[1] - a[0])));
        let mut ret = 0;
        let mut prev_save = 0;
        for task in tasks.iter() {
            let cur_cost = task[0];
            let cur_min = task[1];

            if cur_min > prev_save {
                ret += cur_min - prev_save;

                prev_save = cur_min - cur_cost;
            } else {
                prev_save -= cur_cost;
            }
        }

        ret
    }
}
fn main() {}
