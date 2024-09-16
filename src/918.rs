struct Solution;

impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        if sz == 1 {
            return nums[0];
        }

        fn kadane(nums: &[i32], is_max: bool) -> i32 {
            let mut max_sum = nums[0];
            let mut cur_sum = 0;
            for &num in nums.iter() {
                if is_max {
                    cur_sum = num.max(cur_sum + num);
                    max_sum = max_sum.max(cur_sum);
                } else {
                    cur_sum = num.min(cur_sum + num);
                    max_sum = max_sum.min(cur_sum);
                }
            }
            max_sum
        }
        let max_sum = kadane(&nums, true);
        let min_sum = kadane(&nums, false);
        let total_sum: i32 = nums.iter().sum();
        if max_sum > 0 {
            max_sum.max(total_sum - min_sum)
        } else {
            max_sum
        }
    }
}

fn main() {}
