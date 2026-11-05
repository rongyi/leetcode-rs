struct Solution;

impl Solution {
    const MOD: i64 = 1_000_000_007;
    pub fn ways_to_reach_target(target: i32, types: Vec<Vec<i32>>) -> i32 {
        // [[count, score]...]
        let types: Vec<Vec<i64>> = types
            .into_iter()
            .map(|v| v.into_iter().map(|a| a as i64).collect())
            .collect();
        let mut dp: Vec<Vec<i64>> = vec![vec![-1; target as usize + 1]; types.len() + 1];
        Self::dfs(0, target as i64, &types, &mut dp) as _
    }

    fn dfs(idx: usize, target: i64, types: &Vec<Vec<i64>>, dp: &mut Vec<Vec<i64>>) -> i64 {
        if target == 0 {
            return 1;
        }
        if idx == types.len() {
            if target == 0 {
                return 1;
            } else {
                return 0;
            }
        }
        if dp[idx][target as usize] != -1 {
            return dp[idx][target as usize] % Self::MOD;
        }
        let mut ret = 0;

        let mut i = 0;
        while i <= types[idx][0] && target - types[idx][1] * i >= 0 {
            ret += Self::dfs(idx + 1, target - types[idx][1] * i, types, dp);
            i += 1;
        }

        dp[idx][target as usize] = ret % Self::MOD;

        dp[idx][target as usize]
    }
}

fn main() {}
