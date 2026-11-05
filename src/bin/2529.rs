struct Solution;

impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let mut neg_len = 0;
        let mut pos_len = 0;
        let mut found_nonneg = false;
        for (i, &val) in nums.iter().enumerate() {
            if val >= 0 {
                found_nonneg = true;
                neg_len = i as i32;
                let mut j = i;
                while j < nums.len() && nums[j] == 0 {
                    j += 1;
                }
                pos_len = (nums.len() - j) as i32;
                break;
            }
        }
        if !found_nonneg {
            return nums.len() as _;
        }

        neg_len.max(pos_len)
    }
}

fn main() {}
