struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        Self::at_most_k(&nums, k) - Self::at_most_k(&nums, k - 1)
    }
    fn at_most_k(nums: &Vec<i32>, k: i32) -> i32 {
        let mut ret = 0;
        let mut i = 0;
        let mut count: HashMap<i32, i32> = HashMap::new();

        for j in 0..nums.len() {
            *count.entry(nums[j]).or_insert(0) += 1;
            while count.len() > k as usize {
                let count_left = count.get_mut(&nums[i]).unwrap();
                *count_left -= 1;
                if *count_left == 0 {
                    count.remove(&nums[i]);
                }

                i += 1;
            }
            ret += j - i + 1;
        }

        ret as i32
    }
}

fn main() {}
