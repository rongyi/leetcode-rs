
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn longest_wpi(hours: Vec<i32>) -> i32 {
        let mut score: HashMap<i32, usize> = HashMap::new();
        let mut sum = 0;
        let mut ret = 0;
        for (i, &hour) in hours.iter().enumerate() {
            sum += if hour > 8 { 1 } else { -1 };
            // this total range till i is a valid range
            if sum > 0 {
                ret = i as i32 + 1;
            } else {
                score.entry(sum).or_insert(i);
                if let Some(&j) = score.get(&(sum - 1)) {
                    ret = ret.max(i as i32 - j as i32);
                }
            }
        }
        ret
    }
}

fn main() {}
