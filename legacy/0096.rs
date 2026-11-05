struct Solution;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n: usize = n as usize;
        if n <= 1 {
            return 1;
        }
        let mut dp = vec![0; n + 1];

        // empty
        dp[0] = 1;
        // only one node
        dp[1] = 1;
        for i in 2..=n {
            for j in 1..=i {
                // j as root
                // left child nubmer * right child number
                dp[i] += dp[j - 1] * dp[i - j];
            }
        }

        dp[n]
    }
}

