struct Solution;

impl Solution {
    pub fn largest_sum_after_k_negations(mut nums: Vec<i32>, mut k: i32) -> i32 {
        // Sort in ascending order to bring the most negative numbers to the front
        nums.sort_unstable();

        // Pass 1: Negate negative numbers starting from the smallest (most negative)
        for num in nums.iter_mut() {
            if *num < 0 && k > 0 {
                *num = -*num;
                k -= 1;
            }
        }

        // Pass 2: If k is still odd, flip the smallest value in the modified array
        if k % 2 == 1 {
            if let Some(min_val) = nums.iter_mut().min() {
                *min_val = -*min_val;
            }
        }

        nums.iter().sum()
    }
}

fn main() {}
