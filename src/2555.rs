struct Solution;

impl Solution {
    pub fn maximize_win(prize_positions: Vec<i32>, k: i32) -> i32 {
        let sz = prize_positions.len();
        let mut dp = vec![0; sz + 1];
        let mut ret = 0;
        let mut i = 0;
        for j in 0..sz {
            while prize_positions[i] < prize_positions[j] - k {
                i += 1;
            }
            dp[j + 1] = dp[j].max(j - i + 1);
            ret = ret.max(dp[i] + (j - i + 1));
        }
        ret as _
    }
}

fn main() {}
