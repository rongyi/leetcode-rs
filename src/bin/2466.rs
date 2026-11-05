struct Solution;

impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        let mut ret = 0;

        let mut dp = vec![-1i64; high as usize + 1];

        for i in low..=high {
            ret = (ret + Self::recur(i as _, zero as _, one as _, &mut dp)) % 1_000_000_007;
        }

        ret as _
    }

    fn recur(target: i64, zero: i64, one: i64, dp: &mut Vec<i64>) -> i64 {
        if target == 0 {
            return 1;
        }
        if target < 0 {
            return 0;
        }

        if dp[target as usize] != -1 {
            return dp[target as usize];
        }

        dp[target as usize] = (Self::recur(target - zero, zero, one, dp)
            + Self::recur(target - one, zero, one, dp))
            % 1_000_000_007;

        dp[target as usize]
    }
}

fn main() {}
