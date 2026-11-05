struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let sz = nums.len();
        let threshhold = sz / 3;
        let mut nums = nums;
        nums.sort();
        let mut i = 0;
        let mut ret: Vec<i32> = Vec::new();
        while i < sz {
            let mut j = i;
            while j < sz && nums[j] == nums[i] {
                j += 1;
            }
            if j - i > threshhold {
                ret.push(nums[i]);
            }

            i = j;
        }
        ret
    }
}

fn main() {}
