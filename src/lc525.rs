
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut sum_index: HashMap<i32, i32> = HashMap::new();
        sum_index.insert(0, -1);
        let mut sum = 0;
        let mut ret = 0;
        for (i, &num) in nums.iter().enumerate() {
            sum += if num == 0 { -1 } else { 1 };
            if sum_index.contains_key(&sum) {
                ret = ret.max(i as i32 - sum_index[&sum]);
            } else {
                sum_index.insert(sum, i as i32);
            }
        }

        ret
    }
}

fn main() {}
