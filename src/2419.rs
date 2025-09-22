struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let max_val = *nums.iter().max().unwrap();
        let mut i = 0;
        let sz = nums.len();
        let mut ret = 1;
        while i < sz {
            if nums[i] != max_val {
                i += 1;
            } else {
                let mut j = i + 1;
                while j < sz && nums[j] == nums[j - 1] {
                    j += 1;
                }
                ret = ret.max((j - i) as i32);

                i = j;
            }
        }
        ret
    }
}

fn main() {}
