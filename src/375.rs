struct Solution;

impl Solution {
    pub fn get_money_amount(n: i32) -> i32 {
        let mut dp = vec![vec![0; (n + 1) as usize]; (n + 1) as usize];

        fn recur(l: i32, r: i32, dp: &mut Vec<Vec<i32>>) -> i32 {
            if l >= r {
                return 0;
            }
            if dp[l as usize][r as usize] != 0 {
                return dp[l as usize][r as usize];
            }

            let mut ret = i32::MAX;

            for i in l..=r {
                ret = ret.min(i + std::cmp::max(recur(l, i - 1, dp), recur(i + 1, r, dp)));
            }

            dp[l as usize][r as usize] = ret;

            ret
        }

        recur(1, n, &mut dp)
    }
}

fn main() {}
