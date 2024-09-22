struct Solution;

impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let sz = nums.len();
        let mut left = 0;
        let mut cur_sum = 0;
        let mut ret = 0;
        for i in 0..sz {
            cur_sum += nums[i];
            while left < i && cur_sum > goal {
                cur_sum -= nums[left];
                left += 1;
            }
            if cur_sum < goal {
                continue;
            }
            if cur_sum == goal {
                ret += 1;
            }
            let mut t = left;
            while t < i && nums[t] == 0 {
                t += 1;
                ret += 1;
            }
        }

        ret
    }
}

fn main() {}
