#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn connect_two_groups(cost: Vec<Vec<i32>>) -> i32 {
        let size1 = cost.len();
        let size2 = cost[0].len();

        // Precompute minimum costs for each column (second group)
        let mut min_costs_group2 = vec![i32::MAX; size2];
        for j in 0..size2 {
            for i in 0..size1 {
                min_costs_group2[j] = min_costs_group2[j].min(cost[i][j]);
            }
        }

        // Calculate DP using memoization
        let mut memo = vec![vec![-1; 1 << size2]; size1 + 1];

        Self::dp(&cost, &min_costs_group2, size1, size2, 0, 0, &mut memo)
    }

    fn dp(
        cost: &Vec<Vec<i32>>,
        min_costs_group2: &Vec<i32>,
        size1: usize,
        size2: usize,
        i: usize,
        mask: usize,
        memo: &mut Vec<Vec<i32>>,
    ) -> i32 {
        // If we've completed all rows (first group)
        if i == size1 {
            let mut result = 0;
            // For all unconnected columns, add the minimum cost
            for j in 0..size2 {
                if (mask & (1 << j)) == 0 {
                    result += min_costs_group2[j];
                }
            }
            return result;
        }

        // If we've already computed this state
        if memo[i][mask] != -1 {
            return memo[i][mask];
        }

        let mut result = i32::MAX;

        // Try connecting the current row (i) to each column (j)
        for j in 0..size2 {
            let new_mask = mask | (1 << j);
            let next_cost =
                cost[i][j] + Self::dp(cost, min_costs_group2, size1, size2, i + 1, new_mask, memo);
            result = result.min(next_cost);
        }

        memo[i][mask] = result;
        return result;
    }
}
fn main() {}
