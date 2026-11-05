struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut max_len = 0;

        // Store values in a set for O(1) lookup
        let values: HashSet<i32> = arr.iter().cloned().collect();

        // DP: dp[(i, j)] = length of Fibonacci subsequence ending with arr[i], arr[j]
        // We can use a 2D array or HashMap
        // Using HashMap with encoded index for better memory efficiency
        let mut dp = HashMap::new();

        // For each pair (j, k) where j < k
        for j in 0..n {
            for k in j + 1..n {
                let target = arr[j] + arr[k];
                if values.contains(&target) {
                    // Find the index of target (we know it exists)
                    // We need to find if there's a previous element i such that arr[i] + arr[j] = arr[k]
                    // Actually, we need to find the length ending with arr[j], arr[k]

                    // Check if we can extend a sequence ending with arr[i], arr[j]
                    // where arr[i] + arr[j] = arr[k]
                    // Using HashMap to store dp[(i, j)]
                    let prev = arr[k] - arr[j];
                    if let Some(&prev_len) = dp.get(&(prev, arr[j])) {
                        let new_len = prev_len + 1;
                        dp.insert((arr[j], arr[k]), new_len);
                        max_len = max_len.max(new_len);
                    } else {
                        // Start a new sequence of length 3
                        dp.insert((arr[j], arr[k]), 3);
                        max_len = max_len.max(3);
                    }
                }
            }
        }

        max_len
    }
}

fn main() {}
