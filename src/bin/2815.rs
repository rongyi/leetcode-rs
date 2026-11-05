struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn max_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        // by max digit
        let mut group: HashMap<i32, Vec<i32>> = HashMap::new();
        for &num in nums.iter() {
            let mut max_digit = 0;
            let mut cur = num;
            while cur > 0 {
                let d = cur % 10;
                max_digit = max_digit.max(d);

                cur /= 10;
            }
            group.entry(max_digit).or_default().push(num);
        }
        let mut max_sum = -1;
        for vals in group.values() {
            if vals.len() > 1 {
                let cur_sum = vals[vals.len() - 1] + vals[vals.len() - 2];
                max_sum = max_sum.max(cur_sum);
            }
        }

        max_sum
    }
}

fn main() {}
