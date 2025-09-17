struct Solution;

impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        let mut ret = 1;
        for i in 0..sz {
            let mut j = i + 1;
            let mut acc = nums[i];
            while j < sz {
                if acc & nums[j] == 0 {
                    acc |= nums[j];
                } else {
                    break;
                }

                j += 1;
            }
            ret = ret.max((j - i) as i32);
        }

        ret
    }
}

fn main() {}
