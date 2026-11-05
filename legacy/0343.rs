struct Solution;

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[1] = 1;
        for i in 2..=n {
            for j in 1..i {
                // split i -> j and i - j, and split (i - j) to smaller parts,
                // and its max value can be get is dp[i - j]
                dp[i] = dp[i].max(i32::max((j * (i - j)) as i32, j as i32 * dp[i - j]))
            }
        }

        dp[n]
    }
}

fn main() {}
