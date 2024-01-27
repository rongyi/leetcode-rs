struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let val = nums.iter().fold(0, |acc, &num| acc ^ num);
        let mut ret = vec![0, 0];
        let bit = val & -val;

        for &num in &nums {
            if num & bit == 0 {
                ret[0] ^= num;
            } else {
                ret[1] ^= num;
            }
        }

        ret
    }
}

fn main() {}
