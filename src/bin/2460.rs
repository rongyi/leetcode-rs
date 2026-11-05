struct Solution;

impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                nums[i] *= 2;
                nums[i + 1] = 0;
            }
        }
        let mut non_zero: Vec<i32> = nums.iter().copied().filter(|x| *x > 0).collect();
        let zero = vec![0; nums.len() - non_zero.len()];
        non_zero.extend(zero);

        non_zero
    }
}

fn main() {}
