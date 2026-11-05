#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn can_partition_k_subsets(mut nums: Vec<i32>, k: i32) -> bool {
        let total_sum: i32 = nums.iter().sum();
        if total_sum % k != 0 {
            return false;
        }
        let chunk_sum = total_sum / k;
        let mut visited = vec![false; nums.len()];

        Self::dfs(&nums, &mut visited, k, chunk_sum, 0, 0)
    }

    fn dfs(
        nums: &[i32],
        visited: &mut Vec<bool>,
        k: i32,
        chunk_sum: i32,
        idx: usize,
        cur_sum: i32,
    ) -> bool {
        if k == 1 {
            return true;
        }
        if cur_sum == chunk_sum {
            return Self::dfs(nums, visited, k - 1, chunk_sum, 0, 0);
        }

        for i in idx..nums.len() {
            if !visited[i] {
                visited[i] = true;

                // this line make us pass TLE!
                if cur_sum + nums[i] <= chunk_sum {
                    if Self::dfs(nums, visited, k, chunk_sum, i + 1, cur_sum + nums[i]) {
                        return true;
                    }
                }

                visited[i] = false;
            }
        }

        false
    }
}

fn main() {}
