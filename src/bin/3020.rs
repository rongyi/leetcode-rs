struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        let mut counts = HashMap::new();
        for &num in &nums {
            *counts.entry(num as i64).or_insert(0) += 1;
        }

        let mut max_len = 0;

        // Handle the special case of 1s
        if let Some(&one_count) = counts.get(&1) {
            max_len = if one_count % 2 == 0 {
                one_count - 1
            } else {
                one_count
            };
        }

        for (&num, &count) in &counts {
            if num == 1 {
                continue;
            }

            let mut current_len = 0;
            let mut x = num;

            // Build the chain of squares
            while let Some(&c) = counts.get(&x) {
                if c >= 2 {
                    current_len += 2;
                    // Check for overflow before squaring
                    if x > 100_000 {
                        // sqrt(10^10) is 100,000
                        // Next square would exceed potential i32 limits in input
                        // and we can't find it in the map anyway.
                        // However, the peak MUST be a count of 1 or we stop here.
                        // We subtract 1 because the last element we counted as 2
                        // should have been a peak of 1.
                        current_len -= 1;
                        break;
                    }
                    x = x * x;
                } else {
                    // This is our peak (count == 1)
                    current_len += 1;
                    break;
                }
            }

            // If the loop ended because the next square wasn't in the map,
            // we have an even length (e.g., [x, x]). But the problem
            // requires an odd-length mountain, so the last element found
            // must act as the peak.
            if current_len % 2 == 0 {
                current_len -= 1;
            }

            max_len = max_len.max(current_len);
        }

        max_len as i32
    }
}

fn main() {}
