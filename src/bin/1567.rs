#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn get_max_len(nums: Vec<i32>) -> i32 {
        let mut max_len = 0;
        let mut positive_len = 0;
        let mut negative_len = 0;

        for num in nums {
            if num == 0 {
                // Reset both lengths when encountering 0
                positive_len = 0;
                negative_len = 0;
            } else if num > 0 {
                // Extend positive length
                positive_len += 1;
                // Extend negative length only if there's already a negative sequence
                negative_len = if negative_len > 0 {
                    negative_len + 1
                } else {
                    0
                };
            } else {
                // num < 0
                // Swap and extend: negative becomes positive+1, positive becomes negative+1
                // Example: If nums = [-1,2,3,-4]
                // When we encounter -1: positive_len = 0, negative_len = 1
                // When we encounter 2: positive_len = 1, negative_len = 2
                // When we encounter 3: positive_len = 2, negative_len = 3
                // When we encounter -4: We swap, so positive_len = 4, negative_len = 3
                // When we encounter a negative number, we have two options:
                // 1. Include it in our current sequence, which flips the sign of our product
                // 2. Start a new sequence from this negative number
                // The logic below handles both cases by tracking positive and negative length sequences
                let temp = positive_len;
                positive_len = if negative_len > 0 {
                    negative_len + 1
                } else {
                    0
                };
                negative_len = temp + 1;
            }

            // Update maximum length (we only care about positive product sequences)
            max_len = max_len.max(positive_len);
        }

        max_len
    }
}

fn main() {}
