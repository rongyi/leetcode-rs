struct Solution;

impl Solution {
    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        let mut stack: Vec<usize> = Vec::new();
        let n = nums.len();

        // Step 1: Build a strictly decreasing stack of indices.
        // We only care about decreasing values because if a later value is larger,
        // it would make a narrower ramp than an earlier, smaller value.
        for i in 0..n {
            if stack.is_empty() || nums[i] < nums[*stack.last().unwrap()] {
                stack.push(i);
            }
        }

        let mut max_width = 0;

        // Step 2: Iterate from right to left to maximize width (j - i).
        for j in (0..n).rev() {
            // While the current right-side element is greater than or equal to
            // the element at the top of the stack, it's a valid ramp.
            while let Some(&i) = stack.last() {
                if nums[j] >= nums[i] {
                    max_width = max_width.max(j - i);
                    stack.pop(); // Pop because no smaller `j` can make a wider ramp for this `i`
                } else {
                    break;
                }
            }
        }

        max_width as i32
    }
}

fn main() {}
