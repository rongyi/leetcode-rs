#![allow(dead_code)]

struct Solution;

use std::collections::HashMap;
impl Solution {
    // Finding an increasing subsequence in these indices ensures we're selecting
    // elements from `arr` that appear in the same relative order as they do in `target`
    // which is exactly what defines a common subsequence.
    pub fn min_operations(target: Vec<i32>, arr: Vec<i32>) -> i32 {
        let mut idx: HashMap<i32, usize> = HashMap::new();
        // all uniq
        for (i, &num) in target.iter().enumerate() {
            idx.insert(num, i);
        }
        let mut same: Vec<usize> = Vec::new();

        for &num in &arr {
            if let Some(&i) = idx.get(&num) {
                same.push(i);
            }
        }

        (target.len() - Solution::longest_increasing_subsequence(&same)) as _
    }

    // Function to find the length of the longest increasing subsequence
    fn longest_increasing_subsequence(nums: &[usize]) -> usize {
        let mut sorted = Vec::new();
        for &num in nums.iter() {
            let pos = match sorted.binary_search(&num) {
                Ok(pos) => pos,
                Err(pos) => pos,
            };
            if pos == sorted.len() {
                sorted.push(num);
            } else {
                sorted[pos] = num;
            }
        }

        sorted.len()
    }
}

fn main() {}
