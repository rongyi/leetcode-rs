#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn find_latest_step(arr: Vec<i32>, m: i32) -> i32 {
        let n = arr.len();
        let m = m as usize;

        // If m equals n, the answer is the last step
        if m == n {
            return n as i32;
        }
        // Algorithm Explanation:
        // 1. We track the length of consecutive 1's after each operation
        // 2. For each operation (flipping a bit from 0 to 1):
        //    - We check the lengths of adjacent groups of 1's
        //    - Merge these groups with the new bit
        //    - Update the length information at the boundaries of the new group
        // 3. We maintain a count of groups with exactly length m
        //    - Decrement when existing groups of length m are merged
        //    - Increment when new groups of length m are formed
        // 4. Whenever we have groups of length m, we update the latest step
        // 5. Return the latest step or -1 if no such group exists
        // Track lengths of consecutive 1s
        let mut length = vec![0; n + 2]; // +2 for boundary handling

        // Track count of groups with length m
        let mut count = 0;

        // Track the latest step where a group of size m exists
        let mut latest_step = -1;

        for (step, &pos) in arr.iter().enumerate() {
            let i = pos as usize;

            // Get lengths of adjacent groups
            let left = length[i - 1];
            let right = length[i + 1];

            // New length after merging
            let merged_length = left + right + 1;

            // Update lengths at boundaries of the merged group
            // For each bit flip operation:
            // We check the lengths of adjacent groups (left and right)
            // Then merge these groups with the current position
            // The merged_length is the total length of the new group after merging
            length[i - left] = merged_length;
            length[i + right] = merged_length;
            length[i] = merged_length; // Update at current position

            // Adjust count: decrement for groups that were of size m
            if left == m {
                count -= 1;
            }
            if right == m {
                count -= 1;
            }

            // Add new group if it has the target length
            if merged_length == m {
                count += 1;
            }

            // Update latest_step if groups of size m exist
            if count > 0 {
                latest_step = (step + 1) as i32; // +1 because step is 0-indexed
            }
        }

        latest_step
    }
}

fn main() {}
