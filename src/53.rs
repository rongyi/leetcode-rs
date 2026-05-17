struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut acc = nums[0];
        let mut max_sum = nums[0];
        for &num in nums.iter().skip(1) {
            let nex_acc = num.max(acc + num);
            acc = nex_acc;
            max_sum = max_sum.max(acc);
        }

        max_sum
    }
}

fn main() {}
