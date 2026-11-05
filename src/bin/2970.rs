struct Solution;

impl Solution {
    pub fn incremovable_subarray_count(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = 0;

        // Check all possible subarrays [i, j]
        for i in 0..n {
            for j in i..n {
                // Create the remaining array after removing nums[i..=j]
                let remaining: Vec<i32> = nums[..i]
                    .iter()
                    .chain(nums[j + 1..].iter())
                    .cloned()
                    .collect();

                // Check if remaining is strictly increasing
                let mut is_increasing = true;
                for k in 1..remaining.len() {
                    if remaining[k] <= remaining[k - 1] {
                        is_increasing = false;
                        break;
                    }
                }

                if is_increasing {
                    ans += 1;
                }
            }
        }

        ans
    }
}
fn main() {}
