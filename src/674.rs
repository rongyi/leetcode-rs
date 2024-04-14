struct Solution;

impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut ret = 1;
        for i in 1..nums.len() {
            if nums[i] <= nums[i - 1] {
                ret = ret.max(i - start);
                start = i;
            }
        }
        ret.max(nums.len() - start) as i32
    }
}

fn main() {}
