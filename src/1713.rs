#![allow(dead_code)]
struct Solution;

impl Solution {
    // Finding an increasing subsequence in these indices ensures we're selecting
    // elements from `arr` that appear in the same relative order as they do in `target`
    // which is exactly what defines a common subsequence.
    pub fn min_operations(target: Vec<i32>, arr: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        // Map each value in target to its index
        let val_to_idx: HashMap<i32, usize> = target
            .iter()
            .enumerate()
            .map(|(i, &val)| (val, i))
            .collect();

        // Create a sequence of indices for matching elements in arr
        let mut indices = Vec::new();
        for &val in arr.iter() {
            if let Some(&idx) = val_to_idx.get(&val) {
                indices.push(idx);
            }
        }

        // Find the longest increasing subsequence in indices
        let lis_length = Self::longest_increasing_subsequence(&indices);

        // Return the minimum operations needed
        (target.len() - lis_length) as i32
    }

    // Function to find the length of the longest increasing subsequence
    fn longest_increasing_subsequence(nums: &[usize]) -> usize {
        if nums.is_empty() {
            return 0;
        }

        let mut tails = Vec::new();

        for &num in nums {
            match tails.binary_search(&num) {
                Ok(_) => continue, // Skip duplicates
                Err(pos) => {
                    if pos == tails.len() {
                        tails.push(num);
                    } else {
                        tails[pos] = num;
                    }
                }
            }
        }

        tails.len()
    }
}

fn main() {}
