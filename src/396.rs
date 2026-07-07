struct Solution;

impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        let sum: i32 = nums.iter().sum();
        let mut rotate_sum = 0;
        for (i, num) in nums.iter().enumerate() {
            rotate_sum += num * (i as i32);
        }

        let mut ret = rotate_sum;
        for j in 1..sz {
            rotate_sum += sum - sz as i32 * nums[sz - j];
            ret = ret.max(rotate_sum);
        }

        ret
    }
}

fn main() {}
