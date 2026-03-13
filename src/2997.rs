struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut xor_all = 0;
        for &num in &nums {
            xor_all ^= num;
        }
        let target = xor_all ^ k;

        target.count_ones() as i32
    }
}

fn main() {}
