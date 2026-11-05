#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut initial = 1;
        let mut acc = initial;

        for num in nums.into_iter() {
            let val = acc + num;
            // ok, initial value is not big enough
            if val <= 0 {
                let incr = val.abs() + 1;
                initial += incr;
                acc += incr;
            }
            acc += num;
        }

        initial
    }
}

fn main() {}
