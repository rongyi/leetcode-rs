#![allow(dead_code)]
struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn split_array_same_average(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        let sz = nums.len();
        let m = sz / 2;
        let mut is_possible = false;

        for i in 0..m {
            if is_possible {
                break;
            }
            if sum as usize * (i + 1) % sz == 0 {
                is_possible = true;
            }
        }
        if !is_possible {
            return false;
        }
        let mut sums: Vec<HashSet<i32>> = vec![HashSet::new(); m + 1];
        sums[0].insert(0);

        // vector<vector<unordered_set<int>>> sums(n, vector<unordered_set<int>>(n/2+1));
        // sums[i][j] is all possible combination sum of j numbers from the subarray A[0, i];
        // Goal: sums[n-1][k], for all k in range [1, n/2]
        // Initial condition: sums[i][0] = {0}, 0 <= i <= n-1; sums[0][1] = {all numbers in the array};
        // Deduction: sums[i+1][j] = sums[i][j] "join" (sums[i][j-1] + A[i+1])
        for &num in nums.iter() {
            for i in (1..=m).rev() {
                for &t in sums[i - 1].clone().iter() {
                    sums[i].insert(t + num);
                }
            }
        }
        for i in 1..=m {
            let key = (sum as usize * i / sz) as i32;
            if sum as usize * i % sz == 0 && sums[i].contains(&key) {
                return true;
            }
        }

        false
    }
}

fn main() {}
