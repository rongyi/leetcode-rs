#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = nums.len();

        // Create Pascal's triangle for combinations
        let mut pascal = vec![vec![0i64; n]; n];
        for i in 0..n {
            pascal[i][0] = 1;
            pascal[i][i] = 1;
            for j in 1..i {
                pascal[i][j] = (pascal[i - 1][j - 1] + pascal[i - 1][j]) % MOD;
            }
        }

        // Recursive function to count ways
        fn count_ways(nums: &[i32], pascal: &Vec<Vec<i64>>) -> i64 {
            let n = nums.len();

            // Base cases
            if n <= 2 {
                return 1;
            }

            let root = nums[0];

            // Split into left and right subtrees
            let mut left_subtree = Vec::new();
            let mut right_subtree = Vec::new();

            for &num in nums.iter().skip(1) {
                if num < root {
                    left_subtree.push(num);
                } else {
                    right_subtree.push(num);
                }
            }

            // Recursive count for left and right subtrees
            let left_ways = count_ways(&left_subtree, pascal);
            let right_ways = count_ways(&right_subtree, pascal);

            // Calculate combinations
            let left_size = left_subtree.len();
            let right_size = right_subtree.len();
            let combinations = pascal[left_size + right_size][left_size];

            // Combine results
            // Here we're calculating the total number of ways to arrange the elements
            // while maintaining the same BST structure:
            // 1. combinations: number of ways to interleave left and right subtrees (from Pascal's triangle)
            // 2. left_ways: number of ways to arrange the left subtree (recursive)
            // 3. right_ways: number of ways to arrange the right subtree (recursive)
            // We multiply these values to get the total arrangements, applying modulo at each step to prevent overflow
            return (((combinations * left_ways) % MOD) * right_ways) % MOD;
        }

        // Return result - 1 (as the problem asks for reorderings excluding the original)
        ((count_ways(&nums, &pascal) - 1) % MOD) as i32
    }
}

fn main() {}
