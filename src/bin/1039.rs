struct Solution;

impl Solution {
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        let sz = values.len();
        let mut dp = vec![vec![0; 51]; 51];
        Self::recur(&values, 0, sz - 1, &mut dp);

        dp[0][sz - 1]
    }
    fn recur(values: &Vec<i32>, i: usize, j: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
        if dp[i][j] != 0 {
            return dp[i][j];
        }
        let mut ret = i32::MAX;
        for k in i + 1..j {
            let val = Self::recur(values, i, k, dp)
                + values[i] * values[k] * values[j]
                + Self::recur(values, k, j, dp);
            ret = ret.min(val);
        }
        if ret == i32::MAX {
            ret = 0;
        }
        dp[i][j] = ret;

        dp[i][j]
    }
}

fn main() {}
