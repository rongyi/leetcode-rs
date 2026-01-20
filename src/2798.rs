struct Solution;

impl Solution {
    pub fn number_of_employees_who_met_target(mut hours: Vec<i32>, target: i32) -> i32 {
        hours.sort_unstable();
        let idx = hours.partition_point(|&x| x < target);
        (hours.len() - idx) as i32
    }
}

fn main() {}
