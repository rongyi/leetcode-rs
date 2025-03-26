#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        if n == 1 {
            return vec![1];
        }

        let size = 2 * n - 1;
        let mut result = vec![0; size as usize];
        let mut visited = vec![false; (n + 1) as usize];

        fn backtrack(
            pos: usize,
            result: &mut Vec<i32>,
            visited: &mut Vec<bool>,
            n: i32,
            size: i32,
        ) -> bool {
            // If we've filled the entire sequence, we're done
            if pos == size as usize {
                return true;
            }

            // Skip positions that are already filled
            if result[pos] != 0 {
                return backtrack(pos + 1, result, visited, n, size);
            }

            // Try each number from n down to 1
            for num in (1..=n).rev() {
                if visited[num as usize] {
                    continue;
                }

                // For numbers > 1, we need to check if we can place both occurrences
                if num > 1 {
                    let second_pos = pos + num as usize;

                    // Check if the second position is within bounds and empty
                    if second_pos >= size as usize || result[second_pos] != 0 {
                        continue;
                    }

                    // Place the number at both positions
                    result[pos] = num;
                    result[second_pos] = num;
                    visited[num as usize] = true;

                    // Recursively try to fill the rest
                    if backtrack(pos + 1, result, visited, n, size) {
                        return true;
                    }

                    // Backtrack if unsuccessful
                    result[pos] = 0;
                    result[second_pos] = 0;
                    visited[num as usize] = false;
                } else {
                    // For number 1, we only need one occurrence
                    result[pos] = num;
                    visited[num as usize] = true;

                    // Recursively try to fill the rest
                    if backtrack(pos + 1, result, visited, n, size) {
                        return true;
                    }

                    // Backtrack if unsuccessful
                    result[pos] = 0;
                    visited[num as usize] = false;
                }
            }

            // If no number works, return false to trigger backtracking
            false
        }

        backtrack(0, &mut result, &mut visited, n, size);
        result
    }
}

fn main() {}
