struct Solution;

impl Solution {
    pub fn min_length_after_removals(nums: Vec<i32>) -> i32 {
        let sz = nums.len() as i32;
        let mut max_freq = 0;
        let mut i = 0;
        while i < nums.len() {
            let mut j = i;
            while j < nums.len() && nums[j] == nums[i] {
                j += 1;
            }
            max_freq = max_freq.max((j - i) as i32);
            i = j;
        }
        if max_freq > sz / 2 {
            2 * max_freq - sz
        } else {
            sz % 2
        }
    }
}

fn main() {}
