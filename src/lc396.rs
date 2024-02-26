struct Solution;

impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        let sum = nums.iter().fold(0, |acc, cur| acc + cur);
        let mut rotate_sum = 0;
        for (i, num) in nums.iter().enumerate() {
            rotate_sum += num * (i as i32);
        }

        let mut ret = rotate_sum;
        // you need to find the rotation sum accumation rule
        // e.g. [4, 3, 2, 6]
        // f(0) -> 0 * 4 + 1 * 3 + 2 * 2 + 3 * 6
        // f(1) => 0 * 6 + 1 * 4 + 2 * 3 + 3 * 2 + 4 * 6 - 4 * 6
        // see the rule? get a more sum, and minus 4 * 6 which is sz * nums[sz - j]
        for j in 1..sz {
            rotate_sum += sum - sz as i32 * nums[sz - j];
            ret = ret.max(rotate_sum);
        }

        ret
    }
}

fn main() {}
